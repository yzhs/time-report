use chrono::NaiveDate;
use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use DATE_FORMAT;
use schema::reports;

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "reports"]
pub struct Report {
    pub id: i32,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub was_pdf_generated: bool,
}

/// Create a new report with the given title.
///
/// Insert a new row into the `reports` table with `start_date` set to the first day after the end
/// of the final date of the previous report and `end_date` set to today.
pub fn create_report_from_title<S: AsRef<str>>(conn: &SqliteConnection, title: S) {
    // Find last end date for last report
    let prev_end_date_string = reports::table
        .select(diesel::dsl::max(reports::end_date))
        .first::<Option<String>>(conn)
        .unwrap();
    let start_date = NaiveDate::parse_from_str(
        &prev_end_date_string.unwrap_or_else(|| "2017-12-01".into()),
        DATE_FORMAT,
    ).expect("Invalid date");

    let new_report = (
        reports::title.eq(title.as_ref()),
        reports::start_date.eq(format!("{}", start_date.succ().format(DATE_FORMAT))),
    );

    diesel::insert_into(reports::table)
        .values(&new_report)
        .execute(conn)
        .expect("Failed to create report");
}

pub fn get(conn: &SqliteConnection, id: i32) -> Option<Report> {
    reports::table
        .filter(reports::id.eq(id))
        .first::<Report>(conn)
        .ok()
}

pub fn get_all(conn: &SqliteConnection) -> Vec<Report> {
    reports::table.load::<Report>(conn).unwrap()
}

pub fn find_or_insert_report(conn: &SqliteConnection) -> i32 {
    match reports::table
        .select(reports::id)
        .filter(diesel::dsl::not(reports::was_pdf_generated))
        .first::<i32>(conn)
    {
        Ok(id) => id,
        Err(_) => {
            let values = (reports::title.eq(""), reports::start_date.eq("2017-08-01"));
            diesel::insert_into(reports::table)
                .values(&values)
                .execute(conn)
                .unwrap();
            find_or_insert_report(conn)
        }
    }
}

pub fn add(conn: &SqliteConnection, report: &Report) {
    diesel::insert_into(reports::table)
        .values(report)
        .execute(conn)
        .unwrap();
}

pub fn update(conn: &SqliteConnection, report: &Report) {
    diesel::update(reports::table)
        .filter(reports::id.eq(report.id))
        .set((
            reports::title.eq(&report.title),
            reports::start_date.eq(&report.start_date),
            reports::end_date.eq(&report.end_date),
        ))
        .execute(conn)
        .expect("Failed to update report");
}

#[derive(Serialize)]
struct EmployeeItem {
    date: String,
    type_of_week: i32,
    hours: String,
    minutes: String,
    remark: String,
}

#[derive(Serialize)]
struct PerEmployeeData {
    name: String,
    hours: i32,
    minutes: i32,
    items: Vec<EmployeeItem>,
}

impl PerEmployeeData {
    fn compile(conn: &SqliteConnection, id: i32) -> Self {
        use chrono::Duration;

        use schema::employees;
        use schema::items_view;

        let name = employees::table
            .select(employees::name)
            .filter(employees::id.eq(id))
            .first(conn)
            .unwrap();

        let mut total_time = Duration::zero();

        let items = items_view::table
            .filter(items_view::employee_id.eq(id))
            .load::<::items::InvoiceItem>(conn)
            .unwrap()
            .into_iter()
            .map(|item| {
                let date = format!("{}", item.day.format(DATE_FORMAT));

                let duration = item.end.signed_duration_since(item.start);
                total_time = total_time + duration;

                let hours = format!("{}", duration.num_hours());
                let minutes = format!("{}", duration.num_minutes() % 60);

                EmployeeItem {
                    date,
                    type_of_week: item.type_of_week,
                    hours,
                    minutes,
                    remark: item.remark,
                }
            })
            .collect();

        let hours = total_time.num_hours() as i32;
        let minutes = (total_time.num_minutes() % 60) as i32;

        PerEmployeeData {
            name,
            hours,
            minutes,
            items,
        }
    }
}

#[derive(Serialize)]
pub struct PerEmployeeReport {
    title: String,
    employees: Vec<PerEmployeeData>,
}

impl PerEmployeeReport {
    pub fn generate(conn: &SqliteConnection, id: i32) -> Self {
        use schema::reports;
        use schema::items_view;

        let report_title = reports::table
            .select(reports::title)
            .filter(reports::id.eq(id))
            .first(conn)
            .expect("Could not find report");

        let employee_ids = items_view::table
            .select(items_view::employee_id)
            .group_by(items_view::employee_id)
            .load::<i32>(conn)
            .unwrap();
        info!(
            "The current report contains data concerning {:?}",
            employee_ids
        );

        let employees = employee_ids
            .into_iter()
            .map(|id| PerEmployeeData::compile(conn, id))
            .collect();

        Self {
            title: report_title,
            employees,
        }
    }
}
