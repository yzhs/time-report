use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use errors::*;
use schema::reports;

/// Represent a row in the `reports` table.
#[derive(Debug, Serialize, Deserialize, Insertable, Queryable)]
#[table_name = "reports"]
pub struct Report {
    pub id: i32,
    pub title: String,
    pub start_date: String,
    pub end_date: String,
    pub was_pdf_generated: bool,
}

/// Get data for the report with the given id.
pub fn get(conn: &SqliteConnection, id: i32) -> Result<Report> {
    reports::table
        .filter(reports::id.eq(id))
        .first::<Report>(conn)
        .chain_err(|| format!("Failed to get report #{}", id))
}

/// Get *all* reports from the database.
pub fn get_all(conn: &SqliteConnection) -> Result<Vec<Report>> {
    reports::table
        .load::<Report>(conn)
        .chain_err(|| "Could not load reports table")
}

/// Insert a new report into the database.
pub fn add(conn: &SqliteConnection, report: &Report) -> Result<()> {
    diesel::insert_into(reports::table)
        .values(report)
        .execute(conn)
        .map(|_| ())
        .chain_err(|| format!("Failed to insert new report: {:?}", report))
}

/// Replace a report in the database.
pub fn update(conn: &SqliteConnection, report: &Report) -> Result<()> {
    diesel::update(reports::table)
        .filter(reports::id.eq(report.id))
        .set((
            reports::title.eq(&report.title),
            reports::start_date.eq(&report.start_date),
            reports::end_date.eq(&report.end_date),
        ))
        .execute(conn)
        .map(|_| ())
        .chain_err(|| format!("Failed to update report: {:?}", report))
}

/// One item in the report.
///
/// It belongs to a specific employee. It represents one row in the final PDF report, which is why
/// it only contains strings, and is only used for writing the formatted data to a file.
#[derive(Serialize)]
struct EmployeeItem {
    /// When the employee worked
    date: String,

    /// What type of week was this day in?
    type_of_week: String,

    /// How many full hours did they work?
    hours: String,

    /// How many minutes did they work? (< 60)
    minutes: String,

    /// Whatever remark, e.g. stand-in for so-and-so.
    remark: String,
}

/// Section in the report with all the data for one employee.
#[derive(Serialize)]
struct PerEmployeeData {
    name: String,
    hours: i32,
    minutes: i32,
    items: Vec<EmployeeItem>,
}

impl PerEmployeeData {
    /// Get all data for a specific repord and a specific employee.
    fn compile(conn: &SqliteConnection, report_id: i32, id: i32) -> Result<Self> {
        use chrono::Duration;

        use schema::employees;
        use schema::items_view;

        let name = employees::table
            .select(employees::name)
            .filter(employees::id.eq(id))
            .first(conn)
            .chain_err(|| "Failed to find employee's name")?;

        let mut total_time = Duration::zero();

        let items = items_view::table
            .filter(items_view::report_id.eq(report_id))
            .filter(items_view::employee_id.eq(id))
            .order(items_view::day.asc())
            .load::<::items::InvoiceItem>(conn)
            .chain_err(|| "Failed to query items_view")?
            .into_iter()
            .map(|item| {
                let date = format!("{}", item.day.format("%d.\\,%m.\\,%y"));

                let duration = item.end.signed_duration_since(item.start);
                total_time = total_time + duration;

                let hours = format!("{}", duration.num_hours());
                let minutes = format!("{}", duration.num_minutes() % 60);

                EmployeeItem {
                    date,
                    type_of_week: ::generate_pdf::TYPE_OF_WEEK[item.type_of_week as usize]
                        .to_string(),
                    hours,
                    minutes,
                    remark: item.remark,
                }
            })
            .collect();

        let hours = total_time.num_hours() as i32;
        let minutes = (total_time.num_minutes() % 60) as i32;

        Ok(PerEmployeeData {
            name,
            hours,
            minutes,
            items,
        })
    }
}

/// All the data that goes generating the PDF report.
///
/// This is used to supply the data to the template.
#[derive(Serialize)]
pub struct PerEmployeeReport {
    title: String,
    employees: Vec<PerEmployeeData>,
}

impl PerEmployeeReport {
    /// Read all data for a report from the database.
    // TODO error handling
    pub fn generate(conn: &SqliteConnection, report_id: i32) -> Self {
        use schema::reports;
        use schema::items_view;

        let report_title = reports::table
            .select(reports::title)
            .filter(reports::id.eq(report_id))
            .first(conn)
            .expect("Could not find report");

        let employee_ids = items_view::table
            .filter(items_view::report_id.eq(report_id))
            .select(items_view::employee_id)
            .group_by(items_view::employee_id)
            .order(items_view::name_sort.asc())
            .load::<i32>(conn)
            .unwrap();

        let employees = employee_ids
            .into_iter()
            .map(|id| PerEmployeeData::compile(conn, report_id, id).unwrap())
            .collect();

        Self {
            title: report_title,
            employees,
        }
    }
}

/// For the report with a given id, set `was_pdf_generated` to true.
// TODO error handling?
pub fn set_pdf_generated(conn: &SqliteConnection, id: i32) {
    diesel::update(reports::table.filter(reports::id.eq(id)))
        .set(reports::was_pdf_generated.eq(true))
        .execute(conn)
        .expect("Failed to update was_pdf_generated");
}
