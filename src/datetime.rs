use chrono::{NaiveDate, NaiveTime};
use serde::{Serialize, Deserialize};
use serde::{Serializer, Deserializer};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date(pub NaiveDate);

const DATE_FORMAT: &'static str = "%Y-%d-%d";

impl Date {
    pub fn from_ymd(year: i32, month: u8, day: u8) -> Self {
        Date(NaiveDate::from_ymd(year, month as u32, day as u32))
    }

    pub fn format(&self) -> String {
        format!("{}", self.0.format(DATE_FORMAT))
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
