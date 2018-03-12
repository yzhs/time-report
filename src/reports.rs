use chrono::NaiveDate;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use DATE_FORMAT;
use schema::reports;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "reports"]
pub struct Report {
    pub id: i32,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub was_pdf_generated: bool,
}

/// Create a new report with the given title.
///
/// Insert a new row into the `reports` table with `start_date` set to the first day after the end
/// of the final date of the previous report and `end_date` set to today.
pub fn create_report_from_title<S: AsRef<str>>(conn: &SqliteConnection, title: S) {
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

pub fn get(conn: &SqliteConnection, id: i32) -> Option<Report> {
    reports::table
        .filter(reports::id.eq(id))
        .first::<Report>(conn)
        .ok()
}

pub fn get_all(conn: &SqliteConnection) -> Vec<Report> {
    reports::table.load::<Report>(conn).unwrap()
}

pub fn find_or_insert_report(conn: &SqliteConnection) -> i32 {
    match reports::table
        .select(reports::id)
        .filter(diesel::dsl::not(reports::was_pdf_generated))
        .first::<i32>(conn)
    {
        Ok(id) => id,
        Err(_) => {
            let values = (reports::title.eq(""), reports::start_date.eq("2017-08-01"));
            diesel::insert_into(reports::table)
                .values(&values)
                .execute(conn)
                .unwrap();
            find_or_insert_report(conn)
        }
    }
}

pub fn add(conn: &SqliteConnection, report: Report) {
    diesel::insert_into(reports::table)
        .values(&report)
        .execute(conn)
        .unwrap();
}
