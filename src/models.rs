use super::schema::work_units;

/// Represent one row in the database.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct WorkUnit {
    #[serde(skip)]
    pub id: i32,
    pub name: String,
    pub date: String,
    pub week: i32,
    pub start: String,
    pub end: String,
    pub remark: Option<String>,
    #[serde(default)]
    pub processed: bool,
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
    #[serde(rename = "timePeriod")]
    pub title: Option<String>,

    pub mindate: String,
    pub maxdate: String,

    pub mintime: String,
    pub maxtime: String,
}

impl Globals {
    pub fn new() -> Self {
        use chrono::{Date, Duration, Local, NaiveTime};

        let today = Local::today();
        let half_a_year_ago = today - Duration::weeks(26);
        fn format_date(x: Date<Local>) -> String {
            format!("{}", x.format("%Y-%m-%d"))
        }
        fn format_time(x: NaiveTime) -> String {
            format!("{}", x.format("%H:%M"))
        }
        let mintime = NaiveTime::from_hms(12, 30, 0);
        let maxtime = NaiveTime::from_hms(16, 0, 0);

        Self {
            title: None,
            mindate: format_date(half_a_year_ago), // FIXME compute proper minimal date
            maxdate: format_date(today),
            mintime: format_time(mintime),
            maxtime: format_time(maxtime),
        }
    }
}
