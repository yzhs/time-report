use chrono::{Datelike, NaiveDate};

use super::schema::{items, weeks};

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

/// All the data stored in a row of the main table of the frontend.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewRow {
    pub id: Option<i32>,
    pub name: String,
    pub day: String,
    pub type_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    pub remark: String,
}

/// Data needed to create a new row in the database.
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "items"]
struct NewInvoiceItem {
    pub id: Option<i32>,
    pub employee_id: i32,
    pub report_id: i32,
    pub start_datetime: String,
    pub end_datetime: String,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Insertable)]
#[table_name = "weeks"]
pub struct NewWeek {
    pub year: i32,
    pub week_of_year: i32,
    pub type_of_week: i32,
}

impl NewWeek {
    pub fn new(date: NaiveDate, type_of_week: i32) -> Self {
        let week = date.iso_week();
        Self {
            year: week.year(),
            week_of_year: week.week() as i32,
            type_of_week,
        }
    }
}
