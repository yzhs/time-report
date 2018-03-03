use super::schema::{employees, holidays, items, reports, weeks};

use chrono;
use chrono::Duration;

use datetime::{Date, Time};

/// Represent one row in the database.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct RawInvoiceItem {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub week: i32,
    pub start: String,
    pub end: String,
    pub remark: Option<String>,
    pub processed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceItem {
    pub name: String,
    pub date: Date,
    pub week: i32,
    pub start: Time,
    pub end: Time,
    pub remark: Option<String>,
}

impl InvoiceItem {
    pub fn new() -> Self {
        InvoiceItem {
            name: "".into(),
            date: Date::from_ymd(2017, 8, 1),
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

    pub fn date(mut self, date: Date) -> Self {
        self.date = date;
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

impl From<DbWorkUnit> for WorkUnit {
    fn from(wu: DbWorkUnit) -> Self {
        Self {
            name: wu.name,
            date: wu.date.into(),
            week: wu.week,
            start: wu.start.into(),
            end: wu.end.into(),
            remark: wu.remark,
        }
    }
}

impl From<WorkUnit> for DbWorkUnit {
    fn from(wu: WorkUnit) -> Self {
        Self {
            id: 0,
            name: wu.name,
            date: wu.date.format(),
            week: wu.week,
            start: wu.start.format(),
            end: wu.end.format(),
            remark: wu.remark,
            processed: false,
        }
    }
}

/// Data needed to create a new row in the database.
#[derive(Debug, Deserialize, Insertable)]
#[table_name = "work_units"]
pub struct NewWorkUnit {
    pub name: String,
    pub date: String,
    pub week: i32,
    pub start: String,
    pub end: String,
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
