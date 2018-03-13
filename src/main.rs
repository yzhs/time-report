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
extern crate dotenv_codegen;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

mod api;
pub mod db;
pub mod employees;
pub mod holidays;
pub mod items;
pub mod reports;
pub mod schema;
pub mod weeks;

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

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
        .manage(db::init_pool())
        .mount("/", routes![index, files])
        .mount("/api/", api::routes())
        .attach(options)
        .launch();
}
