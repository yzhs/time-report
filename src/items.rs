use chrono::{NaiveDate, NaiveTime};
use diesel::prelude::*;
use diesel::{self, SqliteConnection};

use employees;
use errors::*;
use schema::items;

use DATE_FORMAT;
use TIME_FORMAT;

lazy_static! {
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
}

/// Get all invoice items from the denormalized `items_view`.
pub fn get(conn: &SqliteConnection, report_id: i32) -> Result<Vec<InvoiceItem>> {
    use schema::items_view;

    assert!(report_id >= 0);

    items_view::table
        .filter(items_view::report_id.eq(report_id))
        .order(items_view::name)
        .order(items_view::day)
        .load::<InvoiceItem>(conn)
        .chain_err(|| {
            format!(
                "Failed to query db for items with report_id = {}",
                report_id
            )
        })
}

/// Generate a reasonable template for the next invoice item.
pub fn template(conn: &SqliteConnection, report_id: i32) -> InvoiceItem {
    use schema::items_view;

    assert!(report_id >= 0);

    match items_view::table
        .filter(items_view::report_id.eq(report_id))
        .order(items_view::day.desc())
        .first::<InvoiceItem>(conn)
    {
        Ok(mut last) => {
            last.id = 0;
            last.name = "".into();
            last
        }
        Err(e) => {
            warn!("Could not find previous item: {:?}", e);
            info!("Using InvoiceItem::new() as a template");
            InvoiceItem::new()
        }
    }
}

/// Update an item with a specific id, or create a new item if `id == 0`.
pub fn update(
    conn: &SqliteConnection,
    report_id: i32,
    id: i32,
    new_row: &NewRow,
) -> Result<InvoiceItem> {
    use schema::{items, items_view};

    assert!(report_id >= 0);
    assert!(id >= 0);

    let employee_id = employees::insert(conn, &new_row.name)
        .chain_err(|| format!("Failed to insert employee: {}", new_row.name))?;

    let date = NaiveDate::parse_from_str(&new_row.day, DATE_FORMAT).chain_err(|| "Invalid date")?;
    let start_time =
        NaiveTime::parse_from_str(&new_row.start_time, TIME_FORMAT).chain_err(|| "Invalid time")?;
    let end_time =
        NaiveTime::parse_from_str(&new_row.end_time, TIME_FORMAT).chain_err(|| "Invalid time")?;
    let start_datetime = date.and_time(start_time);
    let end_datetime = date.and_time(end_time);

    let new_item = (
        items::employee_id.eq(employee_id),
        items::report_id.eq(report_id),
        items::start_datetime.eq(format!("{}", start_datetime)),
        items::end_datetime.eq(format!("{}", end_datetime)),
        items::remark.eq(&new_row.remark),
    );

    if id == 0 {
        // Insert new item
        info!("Creating new item: {:?}", new_row);
        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(conn)
            .chain_err(|| format!("Failed to insert into items table: {:?}", new_item))?;
        items_view::table
            .order(items_view::id.desc())
            .first::<InvoiceItem>(conn)
            .chain_err(|| "Query failed")
    } else {
        // Update existing item
        info!("Updating item #{}: {:?}", id, new_row);
        diesel::update(items::table.filter(items::id.eq(id)))
            .set(new_item.clone())
            .execute(conn)
            .chain_err(|| format!("Failed to update item {:?}", new_item))?;
        items_view::table
            .filter(items_view::id.eq(id))
            .first(conn)
            .chain_err(|| "Query failed")
    }
}
