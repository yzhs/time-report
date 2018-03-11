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
pub mod reports;
pub mod schema;
pub mod weeks;

pub use items::InvoiceItem;

pub mod globals {
    #[derive(Serialize)]
    pub struct Globals {
        pub mintime: &'static str,
        pub maxtime: &'static str,
    }

    impl Globals {
        pub fn new() -> Self {
            Self {
                mintime: "12:30",
                maxtime: "16:00",
            }
        }
    }

    pub fn get() -> Globals {
        Globals::new()
    }
}

pub const DATE_FORMAT: &str = "%Y-%m-%d";
pub const TIME_FORMAT: &str = "%H:%M";
