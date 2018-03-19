use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::{self, SqliteConnection};

use employees;
use holidays;
use schema::items;
use weeks::{get_type_of_week, NewWeek};

use DATE_FORMAT;
use TIME_FORMAT;

lazy_static!{
    static ref START_DEFAULT: NaiveTime = NaiveTime::from_hms(13, 0, 0);
    static ref END_DEFAULT: NaiveTime = NaiveTime::from_hms(15, 30, 0);
}

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

/// An row in `items_view`: Who worked on what day, from when to when.
#[derive(Serialize, Queryable)]
pub struct InvoiceItem {
    pub id: i32,
    pub employee_id: i32,
    pub report_id: i32,
    pub name: String,
    pub name_sort: String,
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
            employee_id: 0,
            report_id: 0,
            name: "".into(),
            name_sort: "".into(),
            day: NaiveDate::from_ymd(2017, 8, 1),
            type_of_week: 0,
            start: *START_DEFAULT,
            end: *END_DEFAULT,
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

    pub fn next(&self) -> Self {
        let next_schoolday = holidays::next_schoolday(self.day);
        InvoiceItem::new()
            .day(next_schoolday)
            .type_of_week(get_type_of_week(next_schoolday))
    }
}

/// Get all invoice items from the denormalized `items_view`.
pub fn get(conn: &SqliteConnection, report_id: i32) -> Vec<InvoiceItem> {
    use schema::items_view;
    items_view::table
        .filter(items_view::report_id.eq(report_id))
        .load::<InvoiceItem>(conn)
        .expect("Error loading data")
}

/// Generate a reasonable template for the next invoice item.
pub fn template(conn: &SqliteConnection, report_id: i32) -> InvoiceItem {
    use schema::items_view;

    match items_view::table
        .filter(items_view::report_id.eq(report_id))
        .order(items_view::day.desc())
        .first::<InvoiceItem>(conn)
    {
        Ok(last) => {
            let mut result = last.next();
            result.id = 0;
            result
        }
        Err(e) => {
            warn!("Could not find previous item: {:?}", e);
            info!("Using InvoiceItem::new() as a template");
            InvoiceItem::new()
        }
    }
}

/// Update an item with a specific id, or create a new item if `id == 0`.
// TODO Use Option<i32>?
pub fn update(conn: &SqliteConnection, report_id: i32, id: i32, new_row: &NewRow) -> i32 {
    use schema::{items, weeks};

    let employee_id = employees::insert(conn, &new_row.name).expect("Failed to find employee");

    let date = NaiveDate::parse_from_str(&new_row.day, DATE_FORMAT).expect("Invalid date");
    let start_time =
        NaiveTime::parse_from_str(&new_row.start_time, TIME_FORMAT).expect("Invalid time");
    let end_time = NaiveTime::parse_from_str(&new_row.end_time, TIME_FORMAT).expect("Invalid time");
    let start_datetime = date.and_time(start_time);
    let end_datetime = date.and_time(end_time);

    let new_week = NewWeek::new(date, new_row.type_of_week);
    diesel::replace_into(weeks::table).values(&new_week);

    if id == 0 {
        // Insert new item
        info!("Creating new item: {:?}", new_row);
        let new_item = (
            items::employee_id.eq(employee_id),
            items::report_id.eq(report_id),
            items::start_datetime.eq(format!("{}", start_datetime)),
            items::end_datetime.eq(format!("{}", end_datetime)),
            items::remark.eq(&new_row.remark),
        );
        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(conn)
            .unwrap();
        items::table
            .select(diesel::dsl::max(items::id))
            .first::<Option<_>>(conn)
            .unwrap()
            .expect("Empty table")
    } else {
        // Update existing item
        info!("Updating item #{}: {:?}", id, new_row);
        let new_item = (
            items::id.eq(id),
            items::employee_id.eq(employee_id),
            items::report_id.eq(report_id),
            items::start_datetime.eq(format!("{}", start_datetime)),
            items::end_datetime.eq(format!("{}", end_datetime)),
            items::remark.eq(&new_row.remark),
        );
        diesel::replace_into(items::table)
            .values(&new_item)
            .execute(conn)
            .unwrap();
        id
    }
}
