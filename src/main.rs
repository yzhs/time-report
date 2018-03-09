#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate time_report;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Json;

use time_report::models::*;
use time_report::reports;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("frontend/dist/index.html")).ok()
}

/// Serve static files
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("frontend/dist/").join(file)).ok()
}

// TODO better name
#[get("/globals", format = "application/json")]
fn get_globals() -> Json<Globals> {
    Json(time_report::get_globals())
}

// TODO better name
#[get("/items", format = "application/json")]
fn get_items() -> Json<Vec<InvoiceItem>> {
    let conn = time_report::establish_connection();
    Json(time_report::get_items(&conn))
}

#[put("/items/<id>", format = "application/json", data = "<item>")]
fn set_item(id: i32, item: Json<NewRow>) -> Json<i32> {
    let conn = time_report::establish_connection();
    Json(time_report::update_item(&conn, id, item.into_inner()))
}

#[get("/new_item", format = "application/json")]
fn new_item() -> Json<InvoiceItem> {
    let conn = time_report::establish_connection();
    Json(time_report::new_item_template(&conn))
}

#[get("/employees", format = "application/json")]
fn get_employees() -> Json<Vec<String>> {
    let conn = time_report::establish_connection();
    Json(time_report::get_employees(&conn))
}

#[get("/holidays", format = "application/json")]
fn get_holidays() -> Json<std::collections::HashMap<String, String>> {
    let conn = time_report::establish_connection();
    Json(time_report::get_holidays(&conn))
}

#[get("/reports", format = "application/json")]
fn get_reports() -> Json<Vec<Report>> {
    let conn = time_report::establish_connection();
    Json(reports::get(&conn))
}

#[post("/reports", format = "application/json", data = "<report>")]
fn add_report(report: Json<Report>) {
    let conn = time_report::establish_connection();
    reports::add(&conn, report.into_inner());
}

fn main() {
    use rocket::http::Method;
    use rocket_cors::{AllowedHeaders, AllowedOrigins};

    let (allowed_origins, _failed_origins) = AllowedOrigins::some(&["http://localhost:8080"]);
    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };
    rocket::ignite()
        .mount("/", routes![index, files])
        .mount(
            "/api/",
            routes![
                get_reports,
                get_globals,
                get_employees,
                get_items,
                get_holidays,
                set_item,
                new_item,
            ],
        )
        .attach(options)
        .launch();
}
