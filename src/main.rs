#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

extern crate time_report;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::Json;

use time_report::models::{WorkUnit, NewWorkUnit};

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
#[get("/rows")]
fn get_rows() -> Json<Vec<WorkUnit>> {
    let conn = time_report::establish_connection();
    Json(time_report::get_rows(&conn))
}


#[post("/rows", format = "application/json", data = "<row>")]
fn post_rows(row: Json<NewWorkUnit>) {
    let conn = time_report::establish_connection();
    time_report::create_row(&conn, row.into_inner());
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/api/", routes![get_rows, post_rows])
        .launch();
}
