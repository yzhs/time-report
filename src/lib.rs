#![recursion_limit = "128"]

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
pub extern crate diesel;

#[macro_use]
extern crate diesel_infer_schema;
extern crate dotenv;

extern crate chrono;

pub mod schema;
pub mod models;

use std::env;

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
        .map(|x| x.into())
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
        result.day(last.day.succ())
    } else {
        result
    }
}

pub fn create_item(conn: &SqliteConnection, new_row: NewRow) {
    use schema::{employees, items, reports, weeks};

    // Create new employee records as needed
    diesel::insert_or_ignore_into(employees::table)
        .values(&employees::name.eq(&new_row.employee_name))
        .execute(conn)
        .expect("Error creating new employee record");

    let employee_id = employees::table
        .select(employees::id)
        .filter(employees::name.eq(&new_row.employee_name))
        .load::<i32>(conn)
        .unwrap()[0];

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
