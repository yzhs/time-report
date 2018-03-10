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

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod db;
pub mod employees;
pub mod holidays;
pub mod items;
pub mod models;
pub mod reports;
pub mod schema;

pub mod globals {
    use models::Globals;
    pub fn get() -> Globals {
        Globals::new()
    }
}
