// Disable warnings caused by Rocket macros.
#![cfg_attr(feature = "clippy", allow(let_unit_value, needless_pass_by_value))]
// `DbConn` has to be passed by value because `Response` is not implemented for `&DbConn`.
#![cfg_attr(feature = "clippy", allow(unit_arg))]

use std::collections::HashMap;

use rocket::response::NamedFile;
use rocket_contrib::Json;

use db;
use errors::*;
use employees;
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
fn set_item(conn: db::DbConn, report_id: i32, id: i32, item: Json<NewRow>) -> Result<Json<i32>> {
    items::update(&conn, report_id, id, &item.into_inner()).map(Json)
}

#[get("/employees", format = "application/json")]
fn get_employees(conn: db::DbConn) -> Json<Vec<String>> {
    Json(employees::get(&conn))
}

#[get("/holidays", format = "application/json")]
fn get_holidays(conn: db::DbConn) -> Json<HashMap<String, String>> {
    Json(holidays::get(&conn))
}

#[get("/reports", format = "application/json")]
fn get_reports(conn: db::DbConn) -> Result<Json<Vec<Report>>> {
    reports::get_all(&conn).map(Json)
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
fn add_report(conn: db::DbConn, report: Json<Report>) -> Result<()> {
    reports::add(&conn, &report.into_inner())
}

#[get("/reports/<id>/pdf/<_filename>")]
fn generate_pdf_report(conn: db::DbConn, id: i32, _filename: String) -> Result<NamedFile> {
    let pdf_file = ::generate_pdf::generate(&conn, id)?;
    NamedFile::open(&pdf_file).chain_err(|| format!("Failed to open file {:?}", pdf_file))
}

pub fn routes() -> Vec<::rocket::Route> {
    routes![
        item_template,
        get_reports,
        get_report,
        put_report,
        get_globals,
        get_employees,
        get_items,
        get_holidays,
        add_report,
        set_item,
        generate_pdf_report,
    ]
}
