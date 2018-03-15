use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use std::iter::FromIterator;
use std::path::Path;
use std::sync::Mutex;

use chrono::{self, Datelike, NaiveDate};
use curl::easy::Easy;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

use schema::holidays;

use DATE_FORMAT;

const GENERAL_HOLIDAYS_URL: &str = "https://feiertage-api.de/api/?nur_land=NW&jahr=";
const SCHOOL_HOLIDAYS_URL: &str = "https://ferien-api.de/api/v1/holidays/NW/";

lazy_static! {
    pub static ref HOLIDAYS: Mutex<HashMap<NaiveDate, String>> = {
        let conn = ::db::connect();
        Mutex::new(get_holidays(&conn))
    };
}

/// Struct for deserializing the JSON document produced by feiertage-api.de.
#[derive(Deserialize)]
struct GeneralHoliday {
    #[serde(rename = "datum")]
    date: NaiveDate,
}

/// Entries of the holidays table.
#[derive(Debug, Serialize, Insertable, Queryable)]
#[table_name = "holidays"]
struct Holiday {
    date: String,
    title: String,
}

/// Represent school holidays as a range with a name. This is used to parse the data produced by
/// ferien-api.de.
#[derive(Deserialize)]
struct SchoolHoliday {
    start: String,
    end: String,
    name: String,
}

fn fetch_url<S: AsRef<str>>(url: S) -> String {
    let mut dst = Vec::new();
    {
        let mut easy = Easy::new();
        easy.url(url.as_ref()).unwrap();

        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            })
            .unwrap();

        transfer.perform().unwrap();
    }

    String::from_utf8_lossy(&dst).into()
}

fn read_general_holidays<S: AsRef<str>>(json: S) -> Vec<Holiday> {
    let map: HashMap<String, GeneralHoliday> =
        ::serde_json::from_str(json.as_ref()).expect("Failed to parse general holidays file");
    map.into_iter()
        .map(|(title, gh)| Holiday {
            title,
            date: format!("{}", gh.date.format(DATE_FORMAT)),
        })
        .collect()
}

fn read_school_holidays<S: AsRef<str>>(json: S) -> Vec<Holiday> {
    let new_holidays: Vec<SchoolHoliday> =
        ::serde_json::from_str(json.as_ref()).expect("Failed to parse school holidays file");

    let mut result = vec![];

    for SchoolHoliday { start, end, name } in new_holidays {
        let start_date = NaiveDate::parse_from_str(&start, "%Y-%m-%dT00:00").unwrap();
        let end_date = NaiveDate::parse_from_str(&end, "%Y-%m-%dT00:00").unwrap();

        let mut dt = start_date;
        while dt <= end_date {
            result.push(Holiday {
                date: format!("{}", dt.format(DATE_FORMAT)),
                title: name.clone(),
            });

            dt = dt.succ();
        }
    }

    result
}

fn store_holidays(conn: &SqliteConnection, new_holidays: &[Holiday]) {
    use schema::holidays;
    ::diesel::replace_into(holidays::table)
        .values(new_holidays)
        .execute(conn)
        .expect("Failed to write holidays to database");
}

/// Get the contents of a file as a String.
fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut content = String::new();
    let mut f = File::open(path)?;
    f.read_to_string(&mut content)
        .expect("Failed to read from file");
    Ok(content)
}

fn read_or_download(base_url: &str, base_path: &str, year: i32) -> String {
    let url = format!("{}{}", base_url, year);
    let path = format!("{}{}.json", base_path, year);
    match read_file(&path) {
        Ok(content) => {
            info!("Successfully read JSON from {}", path);
            content
        }
        Err(e) => {
            info!("Failed to read file: {}", e);
            let content = fetch_url(url);
            let mut f = File::create(path).expect("Could not create file");
            f.write_all(content.as_bytes())
                .expect("Could not write to file");
            content
        }
    }
}

fn add_holidays_for(conn: &SqliteConnection, year: i32) {
    {
        let json = read_or_download(GENERAL_HOLIDAYS_URL, "../feiertage-nrw-", year);
        let new_holidays = read_general_holidays(json);
        store_holidays(conn, &new_holidays);
    }

    {
        let json = read_or_download(SCHOOL_HOLIDAYS_URL, "../ferien-nrw", year);
        let new_school_holidays = read_school_holidays(json);
        store_holidays(conn, &new_school_holidays);
    }

    *HOLIDAYS.lock().unwrap() = get_holidays(conn);
}

pub fn populate_table(conn: &SqliteConnection) {
    let year = ::chrono::Local::today().year();

    add_holidays_for(conn, year);
}

/// Load the entire holidays table.
fn get_holidays(conn: &SqliteConnection) -> HashMap<NaiveDate, String> {
    use schema::holidays;
    HashMap::from_iter(
        holidays::table
            .load::<Holiday>(conn)
            .expect("Failed to read from holidays table")
            .into_iter()
            .map(|holiday| {
                (
                    NaiveDate::parse_from_str(&holiday.date, DATE_FORMAT)
                        .expect(&format!("Invalid date format: {}", holiday.date)),
                    holiday.title,
                )
            }),
    )
}

pub fn is_holiday(date: NaiveDate) -> bool {
    HOLIDAYS.lock().unwrap().contains_key(&date)
}

pub fn next_schoolday(mut date: NaiveDate) -> NaiveDate {
    use chrono::Weekday;
    date = date.succ();
    while date.weekday() == Weekday::Sat || date.weekday() == Weekday::Sun || is_holiday(date) {
        date = date.succ();
    }
    date
}

pub fn get(conn: &SqliteConnection) -> HashMap<String, String> {
    use schema::holidays::*;
    use diesel::dsl::max;

    let last_holiday = NaiveDate::parse_from_str(
        &table
            .select(max(date))
            .first::<Option<String>>(conn)
            .expect("Failed to query holidays table")
            .unwrap_or_else(|| "2017-01-01".into()),
        DATE_FORMAT,
    ).unwrap();
    if last_holiday < chrono::Local::today().naive_local() {
        populate_table(conn);
    }

    HashMap::from_iter(table.load::<(String, String)>(conn).unwrap().into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_populate_holidays_table() {
        let conn = ::db::connect();
        ::diesel::delete(::schema::holidays::table)
            .execute(&conn)
            .unwrap();
        populate_table(&conn);
    }
}
