#![recursion_limit="128"]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

pub mod schema;
pub mod models;

use std::env;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;

use models::{WorkUnit, NewWorkUnit};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!(
        "Error connecting to {}",
        database_url
    ))
}

pub fn get_rows(conn: &SqliteConnection) -> Vec<WorkUnit> {
    use schema::work_units::dsl::work_units;
    work_units.load::<WorkUnit>(conn).expect(
        "Error loading data",
    )
}

pub fn create_row(conn: &SqliteConnection, wu: NewWorkUnit) {
    use schema::work_units;
    diesel::insert(&wu)
        .into(work_units::table)
        .execute(conn)
        .expect("Error saving new row");
}
