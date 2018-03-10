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

#[macro_use]
extern crate log;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod db;
mod employees;
pub mod holidays;
pub mod models;
pub mod reports;
pub mod schema;

pub use employees::*;
use reports::*;
pub use holidays::get_holidays_as_str as get_holidays;

use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use models::*;

pub fn get_globals() -> Globals {
    Globals::new()
}

pub fn get_items(conn: &SqliteConnection) -> Vec<InvoiceItem> {
    use schema::items_view::dsl::items_view;
    items_view
        .load::<InvoiceItem>(conn)
        .expect("Error loading data")
}

pub fn new_item_template(conn: &SqliteConnection) -> InvoiceItem {
    use schema::items_view;

    match items_view::table
        .order(items_view::day.desc())
        .first::<InvoiceItem>(conn)
    {
        Ok(last) => {
            let mut result = last.next();
            result.id = 0;
            result
        }
        Err(e) => {
            info!("Could not find previous item: {:?}", e);
            InvoiceItem::new()
        }
    }
}

fn insert_employee<S: AsRef<str>>(
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

pub fn update_item(conn: &SqliteConnection, id: i32, new_row: NewRow) -> i32 {
    use schema::{items, weeks};

    let employee_id = insert_employee(conn, &new_row.name).expect("Failed to find employee");

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
    let report_id = find_or_insert_report(conn);

    if id == 0 {
        println!(
            "Creating new item: {} {} {} {} {} {}",
            new_row.name,
            new_row.day,
            new_row.type_of_week,
            new_row.start_time,
            new_row.end_time,
            new_row.remark
        );
        // Insert new item
        let new_item = (
            items::employee_id.eq(employee_id),
            items::report_id.eq(report_id),
            items::start_datetime.eq(format!("{}", start_datetime)),
            items::end_datetime.eq(format!("{}", end_datetime)),
            items::remark.eq(new_row.remark),
        );
        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(conn)
            .unwrap();
        items::table
            .select(diesel::dsl::max(items::id))
            .first::<Option<_>>(conn)
            .unwrap()
            .expect("Empty table")
    } else {
        println!(
            "Updating item #{}: {} {} {} {} {} {}",
            id,
            new_row.name,
            new_row.day,
            new_row.type_of_week,
            new_row.start_time,
            new_row.end_time,
            new_row.remark
        );
        // Update existing item
        let new_item = (
            items::id.eq(id),
            items::employee_id.eq(employee_id),
            items::report_id.eq(report_id),
            items::start_datetime.eq(format!("{}", start_datetime)),
            items::end_datetime.eq(format!("{}", end_datetime)),
            items::remark.eq(new_row.remark),
        );
        diesel::replace_into(items::table)
            .values(&new_item)
            .execute(conn)
            .unwrap();
        id
    }
}
