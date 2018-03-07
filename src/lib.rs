#![recursion_limit = "128"]

extern crate chrono;

extern crate curl;

#[macro_use]
pub extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;

extern crate dotenv;

#[macro_use]
extern crate lazy_static;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod schema;
pub mod models;
pub mod holidays;

use std::env;
use std::collections::HashMap;
use std::iter::FromIterator;

use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var(if !cfg!(test) {
        "DATABASE_URL"
    } else {
        "TEST_DATABASE_URL"
    }).expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_globals() -> Globals {
    Globals::new()
}

pub fn get_items(conn: &SqliteConnection) -> Vec<InvoiceItem> {
    use schema::items_view::dsl::items_view;
    items_view
        .load::<InvoiceItem>(conn)
        .expect("Error loading data")
        .into_iter()
        .collect()
}

pub fn new_item_template(conn: &SqliteConnection) -> InvoiceItem {
    let items: Vec<InvoiceItem> = get_items(conn).into_iter().map(|x| x.into()).collect();
    let mut result = InvoiceItem::new();

    if items.is_empty() {
        return result;
    }

    let last = &items[items.len() - 1];
    result = result.day(last.day).type_of_week(last.type_of_week);
    if items.len() == 1 {
        return result;
    }

    let last_but_one = &items[items.len() - 2];
    if last_but_one.day == last.day {
        result.day(holidays::next_schoolday(last.day))
    } else {
        result
    }
}

/// Create a new report with the given title.
///
/// Insert a new row into the `reports` table with `start_date` set to the first day after the end
/// of the final date of the previous report and `end_date` set to today.
pub fn create_report<S: AsRef<str>>(conn: &SqliteConnection, title: S) {
    use schema::reports;

    // Find last end date for last report
    let prev_end_date_string = reports::table
        .select(diesel::dsl::max(reports::end_date))
        .first::<Option<String>>(conn)
        .unwrap();
    let start_date = NaiveDate::parse_from_str(
        &prev_end_date_string.unwrap_or_else(|| "2017-12-01".into()),
        DATE_FORMAT,
    ).expect("Invalid date");

    let new_report = (
        reports::title.eq(title.as_ref()),
        reports::start_date.eq(format!("{}", start_date.succ().format(DATE_FORMAT))),
    );

    diesel::insert_into(reports::table)
        .values(&new_report)
        .execute(conn)
        .expect("Failed to create report");
}

fn create_employee<S: AsRef<str>>(
    conn: &SqliteConnection,
    name: S,
) -> Result<i32, diesel::result::Error> {
    use schema::employees;

    diesel::insert_or_ignore_into(employees::table)
        .values(&employees::name.eq(name.as_ref()))
        .execute(conn)
        .expect("Error creating new employee record");

    employees::table
        .select(employees::id)
        .filter(employees::name.eq(name.as_ref()))
        .first::<i32>(conn)
}

pub fn create_item(conn: &SqliteConnection, new_row: NewRow) {
    use schema::{items, reports, weeks};

    let employee_id =
        create_employee(conn, new_row.employee_name).expect("Failed to find employee");

    let date = NaiveDate::parse_from_str(&new_row.day, DATE_FORMAT).expect("Invalid date");
    let start_time =
        NaiveTime::parse_from_str(&new_row.start_time, TIME_FORMAT).expect("Invalid time");
    let end_time = NaiveTime::parse_from_str(&new_row.end_time, TIME_FORMAT).expect("Invalid time");
    let start_datetime = date.and_time(start_time);
    let end_datetime = date.and_time(end_time);

    // Insert new mapping of week-of-year to type-of-week
    let new_week = NewWeek::new(date, new_row.type_of_week);

    diesel::replace_into(weeks::table).values(&new_week);

    // Get report id
    let report_id = reports::table
        .select(reports::id)
        .filter(diesel::dsl::not(reports::was_pdf_generated))
        .load::<i32>(conn)
        .expect("Failed to find report_id")[0];

    // Insert new item
    let new_item = (
        items::employee_id.eq(employee_id),
        items::report_id.eq(report_id),
        items::start_datetime.eq(format!("{}", start_datetime)),
        items::end_datetime.eq(format!("{}", end_datetime)),
        items::remark.eq(new_row.remark),
    );
    diesel::insert_into(items::table).values(&new_item);
}

pub fn get_employees(conn: &SqliteConnection) -> Vec<String> {
    use schema::employees::*;
    table.select(name).load::<String>(conn).unwrap()
}

pub fn get_holidays(conn: &SqliteConnection) -> HashMap<String, String> {
    use schema::holidays::*;
    use diesel::dsl::max;

    let last_holiday = NaiveDate::parse_from_str(
        &table
            .select(max(date))
            .first::<Option<String>>(conn)
            .expect("Failed to query holidays table")
            .unwrap_or_else(|| "2017-01-01".into()),
        DATE_FORMAT,
    ).unwrap();
    if last_holiday < chrono::Local::today().naive_local() {
        holidays::populate_holidays_table(conn);
    }

    HashMap::from_iter(table.load::<(String, String)>(conn).unwrap().into_iter())
}

#[cfg(test)]
mod test {
    use super::*;

    fn empty_tables(conn: &SqliteConnection) {
        diesel::delete(schema::employees::table)
            .execute(conn)
            .unwrap();
        diesel::delete(schema::items::table).execute(conn).unwrap();
        diesel::delete(schema::holidays::table)
            .execute(conn)
            .unwrap();
        diesel::delete(schema::reports::table)
            .execute(conn)
            .unwrap();
        diesel::delete(schema::weeks::table).execute(conn).unwrap();
    }

    #[test]
    fn test_create_employee() {
        let conn = establish_connection();
        empty_tables(&conn);

        let id = create_employee(&conn, "Alice A.").unwrap();
        let id2 = create_employee(&conn, "Bob B.").unwrap();
        assert_ne!(id, id2);
        assert_eq!(create_employee(&conn, "Alice A.").unwrap(), id);
        assert_eq!(create_employee(&conn, "Bob B.").unwrap(), id2);
    }

    #[test]
    fn test_get_employees() {
        let conn = establish_connection();
        empty_tables(&conn);

        let names = vec!["Alice A.", "Bob B.", "Charlie C."];
        for name in &names {
            create_employee(&conn, name).unwrap();
        }

        assert_eq!(get_employees(&conn), names);
    }
}
