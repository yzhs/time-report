use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Mutex;

use chrono::{Datelike, Local, NaiveDate};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

use errors::*;
use schema::weeks;

lazy_static! {
    static ref TYPE_OF_WEEK: Mutex<TypeOfWeek> = Mutex::new(TypeOfWeek::new(&::db::connect()));
}

/// Names for the different types of weeks.
pub const TYPE_OF_WEEK_NAME: [&str; 4] = ["A", "B", "C", "D"];

/// Map (year, week-of-year) to type-of-week.
struct TypeOfWeek {
    map: HashMap<(i32, i32), i32>,
}

impl TypeOfWeek {
    /// Retrieve the `weeks` table from the database and store it as a labelled set in memory.
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

    /// What type is the week a day is in?
    pub fn get(&self, day: NaiveDate) -> i32 {
        let year = day.year();
        let week_of_year = day.iso_week().week() as i32;
        if self.map.contains_key(&(year, week_of_year)) {
            self.map[&(year, week_of_year)]
        } else {
            // TODO Populate table
            unimplemented!()
        }
    }
}

/// What is the type of the week a given day belongs to?
pub fn get_type_of_week(day: NaiveDate) -> i32 {
    TYPE_OF_WEEK.lock().unwrap().get(day)
}

/// Structure for inserting data into the `weeks` table.
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

/// Compute type of week up to and including the school year starting in the summer of this year.
pub fn populate_table(conn: &SqliteConnection) -> Result<()> {
    use holidays;

    const MIN_YEAR: i32 = 2017;

    let today = Local::today();

    let most_recent_year = {
        use schema::weeks::*;
        table
            .select(year)
            .order(year.desc())
            .first::<i32>(conn)
            .unwrap_or(MIN_YEAR - 1)
    };

    let mut new_weeks = vec![];

    for year in most_recent_year + 1..=today.year() {
        let first_day = holidays::first_day_of_school(conn, year);
        let last_day = holidays::last_day_of_school(conn, year);

        let mut type_of_week = 0;

        let mut prev_week = first_day.iso_week().week() - 1;
        let mut day = first_day;
        while day <= last_day {
            let week = day.iso_week().week();
            if week != prev_week {
                prev_week = week;
                new_weeks.push(NewWeek::new(day, type_of_week));
                type_of_week = (type_of_week + 1) % 4;
            }
            day = holidays::next_schoolday(day);
        }
    }

    ::diesel::insert_into(weeks::table)
        .values(&new_weeks)
        .execute(conn)
        .chain_err(|| "Failed to insert weeks")?;

    Ok(())
}
