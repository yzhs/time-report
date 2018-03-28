// Disable warnings caused by Rocket macros.
#![cfg_attr(feature = "clippy", allow(let_unit_value, needless_pass_by_value))]
// `DbConn` has to be passed by value because `Response` is not implemented for `&DbConn`.
#![cfg_attr(feature = "clippy", allow(unit_arg))]

use std::collections::HashMap;

use chrono::NaiveDate;
use rocket::response::NamedFile;
use rocket_contrib::Json;

use DATE_FORMAT;
use db;
use errors::*;
use employees::{self, Employee};
use items::{self, InvoiceItem, NewRow};
use holidays;
use reports::{self, Report};

#[derive(Serialize)]
pub struct Globals {
    pub mintime: &'static str,
    pub maxtime: &'static str,
}

impl Globals {
    pub fn new() -> Globals {
        Globals {
            mintime: "12:30",
            maxtime: "16:00",
        }
    }
}

#[get("/globals", format = "application/json")]
fn get_globals() -> Json<Globals> {
    Json(Globals::new())
}

#[get("/reports/<report_id>/items", format = "application/json")]
fn get_items(conn: db::DbConn, report_id: i32) -> Result<Json<Vec<InvoiceItem>>> {
    items::get(&conn, report_id).map(Json)
}

#[get("/reports/<report_id>/items/template", format = "application/json")]
fn item_template(conn: db::DbConn, report_id: i32) -> Json<InvoiceItem> {
    Json(items::template(&conn, report_id))
}

#[put("/reports/<report_id>/items/<id>", format = "application/json", data = "<item>")]
fn set_item(
    conn: db::DbConn,
    report_id: i32,
    id: i32,
    item: Json<NewRow>,
) -> Result<Json<InvoiceItem>> {
    items::update(&conn, report_id, id, &item.into_inner()).map(Json)
}

#[get("/employees", format = "application/json")]
fn get_employees(conn: db::DbConn) -> Result<Json<Vec<Employee>>> {
    employees::get(&conn).map(Json)
}

#[put("/employees/<id>", format = "application/json", data = "<employee>")]
fn update_employee(conn: db::DbConn, id: i32, employee: Json<Employee>) -> Result<Json<i32>> {
    warn!("Not implemented");
    Ok(Json(0))
}

#[post("/employees", format = "application/json", data = "<employee>")]
fn add_employee(conn: db::DbConn, employee: Json<Employee>) -> Result<Json<i32>> {
    employees::insert(&conn, employee.into_inner().name).map(Json)
}

#[delete("/employees/<id>")]
fn delete_employee(conn: db::DbConn, id: i32) -> Result<Json<()>> {
    employees::delete(&conn, id).map(Json)
}

#[get("/holidays", format = "application/json")]
fn get_holidays(conn: db::DbConn) -> Json<HashMap<String, String>> {
    Json(holidays::get(&conn))
}

#[get("/reports", format = "application/json")]
fn get_reports(conn: db::DbConn) -> Result<Json<Vec<Report>>> {
    reports::get_all(&conn).map(Json)
}

#[get("/reports/new", format = "application/json")]
fn report_template(conn: db::DbConn) -> Json<Report> {
    Json(reports::template(&conn))
}

#[get("/reports/<id>", format = "application/json")]
fn get_report(conn: db::DbConn, id: i32) -> Result<Json<Report>> {
    reports::get(&conn, id).map(Json)
}

#[put("/reports/<id>", format = "application/json", data = "<report>")]
fn put_report(conn: db::DbConn, id: i32, report: Json<Report>) -> Result<()> {
    let mut report = report.into_inner();
    report.id = id;
    reports::update(&conn, &report)
}

#[post("/reports", format = "application/json", data = "<report>")]
fn add_report(conn: db::DbConn, report: Json<Report>) -> Result<Json<i32>> {
    reports::add(&conn, &report.into_inner()).map(Json)
}

#[get("/reports/<id>/pdf/<_filename>")]
fn generate_pdf_report(conn: db::DbConn, id: i32, _filename: String) -> Result<NamedFile> {
    let pdf_file = ::generate_pdf::generate(&conn, id)?;
    NamedFile::open(&pdf_file).chain_err(|| format!("Failed to open file {:?}", pdf_file))
}

#[get("/next_schoolday/<day>", format = "application/json")]
fn get_next_schoolday(day: String) -> Result<Json<NaiveDate>> {
    NaiveDate::parse_from_str(&day, DATE_FORMAT)
        .chain_err(|| "Invalid date format")
        .map(holidays::next_schoolday)
        .map(Json)
}

#[get("/previous_schoolday/<day>", format = "application/json")]
fn get_previous_schoolday(day: String) -> Result<Json<NaiveDate>> {
    NaiveDate::parse_from_str(&day, DATE_FORMAT)
        .chain_err(|| "Invalid date format")
        .map(holidays::previous_schoolday)
        .map(Json)
}

pub fn routes() -> Vec<::rocket::Route> {
    routes![
        item_template,
        report_template,
        get_reports,
        get_report,
        put_report,
        get_globals,
        get_employees,
        add_employee,
        update_employee,
        delete_employee,
        get_items,
        get_holidays,
        add_report,
        set_item,
        generate_pdf_report,
        get_next_schoolday,
        get_previous_schoolday,
    ]
}
