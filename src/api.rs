// Disable warnings caused by Rocket macros.
#![cfg_attr(feature = "clippy", allow(let_unit_value, needless_pass_by_value))]
// `DbConn` has to be passed by value because `Response` is not implemented for `&DbConn`.
#![cfg_attr(feature = "clippy", allow(unit_arg))]

use std::collections::HashMap;

use rocket_contrib::Json;

use db;
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

#[get("/items", format = "application/json")]
fn get_items(conn: db::DbConn) -> Json<Vec<InvoiceItem>> {
    Json(items::get(&conn))
}

#[get("/items/template", format = "application/json")]
fn item_template(conn: db::DbConn) -> Json<InvoiceItem> {
    Json(items::template(&conn))
}

#[put("/items/<id>", format = "application/json", data = "<item>")]
fn set_item(conn: db::DbConn, id: i32, item: Json<NewRow>) -> Json<i32> {
    Json(items::update(&conn, id, &item.into_inner()))
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
fn get_reports(conn: db::DbConn) -> Json<Vec<Report>> {
    Json(reports::get_all(&conn))
}

#[get("/reports/<id>", format = "application/json")]
fn get_report(conn: db::DbConn, id: i32) -> Option<Json<Report>> {
    reports::get(&conn, id).map(Json)
}

#[post("/reports", format = "application/json", data = "<report>")]
fn add_report(conn: db::DbConn, report: Json<Report>) {
    reports::add(&conn, &report.into_inner());
}

pub fn routes() -> Vec<::rocket::Route> {
    routes![
        item_template,
        get_reports,
        get_report,
        get_globals,
        get_employees,
        get_items,
        get_holidays,
        add_report,
        set_item,
    ]
}
