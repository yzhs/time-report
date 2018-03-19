#![feature(plugin)]
#![plugin(rocket_codegen)]
#![recursion_limit = "128"]

extern crate chrono;

extern crate curl;

// Parse the data from a CSV file
extern crate csv;

#[macro_use]
pub extern crate diesel;
#[macro_use]
extern crate diesel_infer_schema;

extern crate dotenv;
#[macro_use]
extern crate dotenv_codegen;

#[macro_use]
extern crate error_chain;

extern crate handlebars;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

// Create a secure temporary directory to handle the LaTeX side of things.
extern crate tempdir;

use std::path::{Path, PathBuf};

use rocket::http::Method;
use rocket::response::NamedFile;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub mod api;
pub mod db;
pub mod employees;
pub mod generate_pdf;
pub mod holidays;
pub mod items;
pub mod reports;
pub mod schema;
pub mod weeks;

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";

// From https://jamesmunns.com/update/2017/07/22/rocket-plus-error-chain.html
mod errors {
    use std::io::Cursor;

    use rocket::http::{ContentType, Status};
    use rocket::request::Request;
    use rocket::response::{Responder, Response};

    // This generates basic Error, Result, etc. types
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
            Json(::serde_json::Error);
            Diesel(::diesel::result::Error);
        }
    }

    impl<'r> Responder<'r> for Error {
        fn respond_to(self, _: &Request) -> ::std::result::Result<Response<'r>, Status> {
            let mut msg = "".to_string();
            msg.push_str(&format!("Error: {}", self));
            for err in self.iter().skip(1) {
                msg.push_str(&format!(", caused by: {}", err));
            }

            let resp = json!({
                "status": "failure",
                "message": msg,
            }).to_string();

            // Respond. The `Ok` here is a bit of a misnomer. It means we
            // successfully created an error response
            Ok(Response::build()
                .status(Status::BadRequest)
                .header(ContentType::JSON)
                .sized_body(Cursor::new(resp))
                .finalize())
        }
    }
}

use errors::*;

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open(Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/frontend/dist/index.html"
    ))).expect("index.html is missing")
}

/// Serve static files.
///
/// Use rank so as not to conflict with the API endpoints, which also gmatch the pattern of this
/// route. Essentially, that makes this route a fallback.
#[get("/<file..>", rank = 2)]
fn files(file: PathBuf) -> Result<NamedFile> {
    NamedFile::open(Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/frontend/dist/")).join(&file))
        .chain_err(|| format!("File not found: {:?}", file))
}

fn run() -> Result<()> {
    let (allowed_origins, _failed_origins) = AllowedOrigins::some(&["http://localhost:8080"]);
    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Put]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    };

    let rocket = rocket::ignite()
        .manage(db::init_pool())
        .mount("/", routes![index, files])
        .mount("/api/", api::routes())
        .attach(options);

    let conn = db::connect();
    holidays::populate_table(&conn);
    weeks::populate_table(&conn);

    rocket.launch();

    Ok(())
}

// From error-chain's `quickstart` example
// https://github.com/rust-lang-nursery/error-chain/blob/master/examples/quickstart.rs
fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        use error_chain::ChainedError; // trait which holds `display_chain`
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "{}", e.display_chain()).expect(errmsg);
        ::std::process::exit(1);
    }
}
