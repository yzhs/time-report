#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

extern crate time_report;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Json;

use time_report::models::*;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("frontend/dist/index.html")).ok()
}

/// Handle static files
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
#[get("/rows", format = "application/json")]
fn get_rows() -> Json<Vec<DbWorkUnit>> {
    let conn = time_report::establish_connection();
    Json(time_report::get_rows(&conn))
}


#[post("/rows", format = "application/json", data = "<row>")]
fn post_rows(row: Json<NewWorkUnit>) {
    let conn = time_report::establish_connection();
    time_report::create_row(&conn, row.into_inner());
}

#[get("/new_row", format = "application/json")]
fn new_row() -> Json<DbWorkUnit> {
    let conn = time_report::establish_connection();
    Json(time_report::new_row_template(&conn))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/api/", routes![get_globals, get_rows, post_rows, new_row])
        .launch();
}
