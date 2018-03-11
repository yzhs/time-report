use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Mutex;

use chrono::{Datelike, NaiveDate};
use diesel::{RunQueryDsl, SqliteConnection};

use super::schema::weeks;

pub use reports::Report;

lazy_static!{
    static ref TYPE_OF_WEEK: Mutex<TypeOfWeek> =
        Mutex::new(TypeOfWeek::new(&::db::connect()));
}

struct TypeOfWeek {
    map: HashMap<(i32, i32), i32>,
}

impl TypeOfWeek {
    pub fn new(conn: &SqliteConnection) -> Self {
        let map = HashMap::from_iter(
            weeks::table
                .load::<(i32, i32, i32)>(conn)
                .expect("Failed to query type_of_week")
                .into_iter()
                .map(|(year, week, typ)| ((year, week), typ)),
        );
        Self { map }
    }

    pub fn get(&self, day: NaiveDate) -> i32 {
        let year = day.year();
        let week_of_year = day.iso_week().week() as i32;
        if self.map.contains_key(&(year, week_of_year)) {
            self.map[&(year, week_of_year)]
        } else {
            // TODO take a guess
            0
        }
    }
}

pub fn get_type_of_week(day: NaiveDate) -> i32 {
    TYPE_OF_WEEK.lock().unwrap().get(day)
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
