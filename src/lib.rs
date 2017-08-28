#![recursion_limit="128"]

extern crate serde;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

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
use chrono::{Datelike, Duration, NaiveDate};

use models::*;
use datetime::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ))
}

pub fn get_globals() -> Globals {
    Globals::new()
}

pub fn get_rows(conn: &SqliteConnection) -> Vec<DbWorkUnit> {
    use schema::work_units::dsl::work_units;
    work_units.load::<DbWorkUnit>(conn).expect(
        "Error loading data",
    )
}

fn is_work_day(date: NaiveDate) -> bool {
    use chrono::Weekday::*;
    match date.weekday() {
        Sat | Sun => return false,
        _ => {}
    }

    // TODO handle other holidays
    if date.month() == 10 && date.day() == 3 {
        return false;
    }

    true
}

fn next_date(date: Date) -> Date {
    let day = Duration::days(1);
    let mut new_date = date.0 + day;
    while !is_work_day(new_date) {
        new_date = new_date + day;
    }
    Date(new_date)
}

pub fn new_row_template(conn: &SqliteConnection) -> DbWorkUnit {
    let rows: Vec<WorkUnit> = get_rows(conn).into_iter().map(|x| x.into()).collect();
    let mut result = WorkUnit::new();

    if rows.is_empty() {
        return result.into();
    }

    let last = &rows[rows.len() - 1];
    result = result.date(last.date).week(last.week);
    if rows.len() == 1 {
        return result.into();
    }

    let last_but_one = &rows[rows.len() - 2];
    if last_but_one.date == last.date {
        result.date(next_date(last.date)).into()
    } else {
        result.into()
    }
}

pub fn create_row(conn: &SqliteConnection, wu: NewWorkUnit) {
    use schema::work_units;
    diesel::insert(&wu)
        .into(work_units::table)
        .execute(conn)
        .expect("Error saving new row");
}
