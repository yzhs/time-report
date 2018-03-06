use std::collections::HashMap;

use chrono::{Datelike, NaiveDate, NaiveDateTime};
use curl::easy::Easy;
use diesel::{RunQueryDsl, SqliteConnection};

use models::DATE_FORMAT;
use schema::holidays;

const GENERAL_HOLIDAYS_URL: &str = "https://feiertage-api.de/api/?nur_land=NW&jahr=";
const SCHOOL_HOLIDAYS_URL: &str = "https://ferien-api.de/api/v1/holidays/NW/";

#[derive(Debug, Deserialize)]
struct GeneralHoliday {
    #[serde(rename = "datum")]
    date: NaiveDate,

    #[serde(rename = "hinweis")]
    remark: String,
}

#[derive(Debug, Serialize, Insertable)]
#[table_name = "holidays"]
struct Holiday {
    title: String,
    date: String,
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

fn store_holidays(conn: &SqliteConnection, new_holidays: &[Holiday]) {
    use schema::holidays;
    ::diesel::insert_or_ignore_into(holidays::table)
        .values(new_holidays)
        .execute(conn)
        .expect("Failed to write holidays to database");
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

#[derive(Deserialize)]
struct SchoolHoliday {
    start: String,
    end: String,
    name: String,
}

fn read_school_holidays<S: AsRef<str>>(json: S) -> Vec<Holiday> {
    let new_holidays: Vec<SchoolHoliday> =
        ::serde_json::from_str(json.as_ref()).expect("Failed to parse school holidays file");

    let mut result = vec![];

    for SchoolHoliday { start, end, name } in new_holidays.into_iter() {
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

pub fn populate_holidays_table(conn: &SqliteConnection) {
    let year = ::chrono::Local::today().year();

    {
        let json = if !cfg!(test) {
            fetch_url(format!("{}{}", GENERAL_HOLIDAYS_URL, year))
        } else {
            include_str!("../feiertage-nrw-2018.json").into()
        };
        let new_holidays = read_general_holidays(json);
        store_holidays(conn, &new_holidays);
    }

    let json = if !cfg!(test) {
        fetch_url(format!("{}{}", SCHOOL_HOLIDAYS_URL, year))
    } else {
        include_str!("../ferien-nrw-2018.json").into()
    };
    let new_school_holidays = read_school_holidays(json);
    store_holidays(conn, &new_school_holidays);
    println!("{:?}", new_school_holidays);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_holidays() {
        let conn = ::establish_connection();
        populate_holidays_table(&conn);
        assert!(false);
    }
}
