use super::schema::{items, weeks};

use chrono::{Datelike, Duration, NaiveDate, NaiveTime};

pub const DATE_FORMAT: &'static str = "%Y-%m-%d";
pub const TIME_FORMAT: &'static str = "%H:%M";

#[derive(Debug, Serialize, Queryable)]
pub struct InvoiceItem {
    pub id: i32,
    pub name: String,
    pub day: NaiveDate,
    pub type_of_week: i32,
    pub start: NaiveTime,
    pub end: NaiveTime,
    pub remark: String,
}

impl InvoiceItem {
    pub fn new() -> Self {
        InvoiceItem {
            id: 0,
            name: "".into(),
            day: NaiveDate::from_ymd(2017, 8, 1),
            type_of_week: 0,
            start: NaiveTime::from_hms(13, 00, 0),
            end: NaiveTime::from_hms(15, 30, 0),
            remark: "".into(),
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn day(mut self, day: NaiveDate) -> Self {
        self.day = day;
        self
    }

    pub fn type_of_week(mut self, type_of_week: i32) -> Self {
        self.type_of_week = type_of_week;
        self
    }

    pub fn start(mut self, start: NaiveTime) -> Self {
        self.start = start;
        self
    }

    pub fn remark(mut self, remark: &str) -> Self {
        self.remark = remark.into();
        self
    }
}

/// All the data stored in a row of the main table of the frontend.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewRow {
    pub employee_name: String,
    pub day: String,
    pub type_of_week: i32,
    pub start_time: String,
    pub end_time: String,
    pub remark: String,
}

/// Data needed to create a new row in the database.
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "items"]
pub struct NewInvoiceItem {
    pub id: Option<i32>,
    pub employee_id: i32,
    pub report_id: i32,
    pub start_datetime: String,
    pub end_datetime: String,
    pub remark: Option<String>,
}

#[derive(Serialize)]
pub struct Globals {
    pub title: Option<String>,

    pub mindate: String,
    pub maxdate: String,

    pub mintime: String,
    pub maxtime: String,
}

impl Globals {
    pub fn new() -> Self {
        let today = ::chrono::Local::today();
        let today_naive = NaiveDate::from_ymd(today.year(), today.month(), today.day());
        // FIXME compute proper minimal date
        let half_a_year_ago = today_naive - Duration::weeks(26);
        let mintime = NaiveTime::from_hms(12, 30, 0);
        let maxtime = NaiveTime::from_hms(16, 0, 0);

        Self {
            title: None,
            mindate: format!("{}", half_a_year_ago.format(DATE_FORMAT)),
            maxdate: format!("{}", today_naive.format(DATE_FORMAT)),
            mintime: format!("{}", mintime.format(TIME_FORMAT)),
            maxtime: format!("{}", maxtime.format(TIME_FORMAT)),
        }
    }
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
