use super::schema::{employees, holidays, items, reports, weeks};

use chrono;
use chrono::Duration;

use datetime::{Date, Time};

/// Get a list of employees
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Employee(pub String);

/// Represent one row in the database.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct RawInvoiceItem {
    pub id: i32,
    pub name: String,
    pub day: i32,
    pub week: i32,
    pub start: i32,
    pub end: i32,
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub name: String,
    pub day: Date,
    pub week: i32,
    pub start: Time,
    pub end: Time,
    pub remark: Option<String>,
}

impl InvoiceItem {
    pub fn new() -> Self {
        InvoiceItem {
            name: "".into(),
            day: Date::from_ymd(2017, 8, 1),
            week: 0,
            start: Time::from_hms(13, 00, 0),
            end: Time::from_hms(15, 30, 0),
            remark: None,
        }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    pub fn day(mut self, day: Date) -> Self {
        self.day = day;
        self
    }

    pub fn week(mut self, week: i32) -> Self {
        self.week = week;
        self
    }

    pub fn start(mut self, start: Time) -> Self {
        self.start = start;
        self
    }

    pub fn remark(mut self, remark: &str) -> Self {
        self.remark = Some(remark.into());
        self
    }
}

impl From<RawInvoiceItem> for InvoiceItem {
    fn from(wu: RawInvoiceItem) -> Self {
        Self {
            name: wu.name,
            day: wu.day.into(),
            week: wu.week,
            start: wu.start.into(),
            end: wu.end.into(),
            remark: Some(wu.remark),
        }
    }
}

impl From<InvoiceItem> for RawInvoiceItem {
    fn from(wu: InvoiceItem) -> Self {
        Self {
            id: 0,
            name: wu.name,
            day: wu.day.format(),
            week: wu.week,
            start: wu.start.format(),
            end: wu.end.format(),
            remark: wu.remark.unwrap(),
        }
    }
}

/// Data needed to create a new row in the database.
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "items"]
pub struct NewInvoiceItem {
    pub employee_id: i32,
    pub report_id: i32,
    pub start_datetime: i32,
    pub end_datetime: i32,
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
        use chrono::Datelike;
        let today = chrono::Local::today();
        let today_naive = Date::from_ymd(today.year(), today.month() as u8, today.day() as u8);
        let half_a_year_ago = Date(today_naive.0 - Duration::weeks(26));
        let mintime = Time::from_hms(12, 30, 0);
        let maxtime = Time::from_hms(16, 0, 0);

        Self {
            title: None,
            mindate: half_a_year_ago.format(), // FIXME compute proper minimal date
            maxdate: today_naive.format(),
            mintime: mintime.format(),
            maxtime: maxtime.format(),
        }
    }
}
