use super::schema::work_units;

use chrono;
use chrono::{Duration, NaiveDate, NaiveTime};

pub type Date = NaiveDate;
pub type Time = NaiveTime;

pub fn format_date(x: Date) -> String {
    format!("{}", x.format("%Y-%m-%d"))
}

pub fn format_time(x: Time) -> String {
    format!("{}", x.format("%H:%M"))
}

/// Represent one row in the database.
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct DbWorkUnit {
    pub id: i32,
    pub name: String,
    pub date: String,
    pub week: i32,
    pub start: String,
    pub end: String,
    pub remark: Option<String>,
    pub processed: bool,
}

pub struct WorkUnit {
    pub name: String,
    pub date: Date,
    pub week: i32,
    pub start: Time,
    pub end: Time,
    pub remark: Option<String>,
}

impl WorkUnit {
    pub fn new() -> Self {
        WorkUnit {
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
            date: parse_date(&wu.date),
            week: wu.week,
            start: parse_time(&wu.start),
            end: parse_time(&wu.end),
            remark: wu.remark,
        }
    }
}

pub fn parse_date(x: &str) -> Date {
    x.parse::<Date>().expect(
        &format!("Parsing date {} failed", x),
    )
}

pub fn parse_time(x: &str) -> Time {
    chrono::NaiveTime::parse_from_str(x, "%H:%M").expect(&format!("Parsing time {} failed", x))
}

impl From<WorkUnit> for DbWorkUnit {
    fn from(wu: WorkUnit) -> Self {
        Self {
            id: 0,
            name: wu.name,
            date: format_date(wu.date),
            week: wu.week,
            start: format_time(wu.start),
            end: format_time(wu.end),
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
        let today_naive = NaiveDate::from_ymd(today.year(), today.month(), today.day());
        let half_a_year_ago = today_naive - Duration::weeks(26);
        let mintime = NaiveTime::from_hms(12, 30, 0);
        let maxtime = NaiveTime::from_hms(16, 0, 0);

        Self {
            title: None,
            mindate: format_date(half_a_year_ago), // FIXME compute proper minimal date
            maxdate: format_date(today_naive),
            mintime: format_time(mintime),
            maxtime: format_time(maxtime),
        }
    }
}
