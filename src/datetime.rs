use chrono::{Datelike, Duration, NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};
use serde::{Serializer, Deserializer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date(pub NaiveDate);

const DATE_FORMAT: &'static str = "%Y-%m-%d";

fn is_work_day(date: NaiveDate) -> bool {
    use chrono::Weekday::*;
    match date.weekday() {
        Sat | Sun => return false,
        _ => {}
    }

    // TODO handle other holidays
    if date.month() == 10 && date.day() == 3 {
        return false;
    }

    true
}


impl Date {
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        Date(NaiveDate::from_ymd(year, month as u32, day as u32))
    }

    pub fn format(&self) -> String {
        format!("{}", self.0.format(DATE_FORMAT))
    }

    pub fn next(&self) -> Date {
        let day = Duration::days(1);
        let mut new_date = self.0 + day;
        while !is_work_day(new_date) {
            new_date = new_date + day;
        }
        Date(new_date)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = self.format();
        serializer.serialize_str(&string)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Date(
            NaiveDate::parse_from_str(&s, TIME_FORMAT).expect(&format!(
                "Could not parse time {}",
                s
            )),
        ))
    }
}

impl From<String> for Date {
    fn from(x: String) -> Date {
        Date(x.parse::<NaiveDate>().expect(
            &format!("Parsing date {} failed", x),
        ))
    }
}


const TIME_FORMAT: &'static str = "%H:%M";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Time(pub NaiveTime);

impl Time {
    pub fn from_hms(hours: u8, minutes: u8, seconds: u8) -> Self {
        Time(NaiveTime::from_hms(
            hours as u32,
            minutes as u32,
            seconds as u32,
        ))
    }

    pub fn format(&self) -> String {
        format!("{}", self.0.format(TIME_FORMAT))
    }
}

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = self.format();
        serializer.serialize_str(&string)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Time(
            NaiveTime::parse_from_str(&s, TIME_FORMAT).expect(&format!(
                "Could not parse time {}",
                s
            )),
        ))
    }
}

impl From<String> for Time {
    fn from(x: String) -> Time {
        Time(NaiveTime::parse_from_str(&x, "%H:%M").expect(&format!(
            "Parsing time {} failed",
            x
        )))
    }
}
