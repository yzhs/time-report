#![recursion_limit = "128"]

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
pub extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

extern crate chrono;

pub mod schema;
pub mod models;
mod datetime;

use std::env;

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

pub fn get_rows(conn: &SqliteConnection) -> Vec<InvoiceItem> {
    use schema::items_view::dsl::items_view;
    items_view
        .load::<RawInvoiceItem>(conn)
        .expect("Error loading data")
        .into_iter()
        .map(|x| x.into())
        .collect()
}

pub fn new_row_template(conn: &SqliteConnection) -> InvoiceItem {
    let rows: Vec<InvoiceItem> = get_rows(conn).into_iter().map(|x| x.into()).collect();
    let mut result = InvoiceItem::new();

    if rows.is_empty() {
        return result;
    }

    let last = &rows[rows.len() - 1];
    result = result.day(last.day).week(last.week);
    if rows.len() == 1 {
        return result;
    }

    let last_but_one = &rows[rows.len() - 2];
    if last_but_one.day == last.day {
        result.day(last.day.next())
    } else {
        result
    }
}

pub fn create_row(conn: &SqliteConnection, wu: NewInvoiceItem) {
    use schema::items;
    diesel::insert(&wu)
        .into(work_units::table)
        .execute(conn)
        .expect("Error saving new row");
}
