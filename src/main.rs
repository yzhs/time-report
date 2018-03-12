#![feature(plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit = "128"]

extern crate chrono;

extern crate curl;

#[macro_use]
pub extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;

extern crate dotenv;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Json;

use items::{InvoiceItem, NewRow};
use reports::Report;

pub mod db;
pub mod employees;
pub mod holidays;
pub mod items;
pub mod reports;
pub mod schema;
pub mod weeks;

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

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

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/frontend/dist/index.html"
    ))).ok()
}

/// Serve static files
#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/dist/")).join(file))
        .ok()
}

// TODO better name
#[get("/globals", format = "application/json")]
fn get_globals() -> Json<Globals> {
    Json(Globals::new())
}

// TODO better name
#[get("/items", format = "application/json")]
fn get_items() -> Json<Vec<InvoiceItem>> {
    let conn = db::connect();
    Json(items::get(&conn))
}

#[get("/items/template", format = "application/json")]
fn item_template() -> Json<InvoiceItem> {
    let conn = db::connect();
    Json(items::template(&conn))
}

#[put("/items/<id>", format = "application/json", data = "<item>")]
fn set_item(id: i32, item: Json<NewRow>) -> Json<i32> {
    let conn = db::connect();
    Json(items::update(&conn, id, item.into_inner()))
}

#[get("/employees", format = "application/json")]
fn get_employees() -> Json<Vec<String>> {
    let conn = db::connect();
    Json(employees::get(&conn))
}

#[get("/holidays", format = "application/json")]
fn get_holidays() -> Json<std::collections::HashMap<String, String>> {
    let conn = db::connect();
    Json(holidays::get(&conn))
}

#[get("/reports", format = "application/json")]
fn get_reports() -> Json<Vec<Report>> {
    Json(reports::get_all(&db::connect()))
}

#[get("/reports/<id>", format = "application/json")]
fn get_report(id: i32) -> Option<Json<Report>> {
    reports::get(&db::connect(), id).map(|x| Json(x))
}

#[post("/reports", format = "application/json", data = "<report>")]
fn add_report(report: Json<Report>) {
    let conn = db::connect();
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
                item_template,
                get_reports,
                get_report,
                get_globals,
                get_employees,
                get_items,
                get_holidays,
                set_item,
            ],
        )
        .attach(options)
        .launch();
}
