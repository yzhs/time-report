use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

use csv;
use diesel::SqliteConnection;
use tempdir::TempDir;

use items::InvoiceItem;
use reports::{self, Report};
use DATE_FORMAT;
use TIME_FORMAT;

/// Represents a row in the table containing the date a particular person worked on, how many
/// hours and minutes they worked, and a free-text comment filed.
///
/// Each of these structures is a row in the generated table, but not every row in the table
/// represents a Line struct.
#[derive(Debug)]
pub struct Line {
    date: String, // TODO use a proper datatype (time::something?)
    hours: u16,
    minutes: u16,
    remark: String,
}

/// Data related to one person:  Their name, how long they worked in the accounting period and the
/// specific data of when the worked.  Each of these structures is rendered as one table.
#[derive(Debug)]
pub struct Worker {
    name: String, // TODO separate last name?
    hours: u32,
    minutes: u32,
    lines: Vec<Line>,
}

impl Worker {
    pub fn new(name: String) -> Worker {
        Worker {
            name,
            hours: 0,
            minutes: 0,
            lines: vec![],
        }
    }

    pub fn last_name(&self) -> String {
        let name = &self.name;
        if name.contains(',') {
            name.split(',').next()
        } else {
            name.split(' ').last()
        }.unwrap()
            .to_string()
    }
}

impl Line {
    pub fn to_latex(&self) -> String {
        format!(
            "    & {} & {} & {} & {}\\\\\n",
            self.date.replace(".", ".\\,"),
            self.hours,
            self.minutes,
            self.remark
        )
    }

    pub fn check_data(&self) {
        if self.hours > 4 {
            warn!("Hours > 4 in {:?}", self);
        }

        let remark: Vec<_> = self.remark.chars().collect();
        if remark.is_empty() {
            warn!("Remark is empty in {:?}", self);
        } else if !"ABCD".contains(remark[0]) {
            warn!("Remark does not start with the week (A-D): {:?}", self);
        } else if remark.len() > 1 && remark[1] != ' ' {
            warn!(
                "Remark field does not separate week from comment: {:?}",
                self
            );
        }
    }
}

pub struct FullReport {
    metadata: Report,
    items: Vec<InvoiceItem>,
}

#[derive(Debug)]
pub struct MyError;

const TYPE_OF_WEEK: [&str; 4] = ["A", "B", "C", "D"];

impl FullReport {
    fn from_id(conn: &SqliteConnection, id: i32) -> Result<FullReport, MyError> {
        let metadata = reports::get(conn, id).ok_or(MyError)?;
        let items = vec![];
        Ok(FullReport { metadata, items })
    }

    fn write_to_csv(&self) -> ::csv::Result<()> {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("csv")
            .join(&self.metadata.title);
        path.set_extension("csv");

        let mut writer = ::csv::Writer::from_file(path).expect("Failed to create CSV writer");

        for item in &self.items {
            let row = (
                &item.name,
                format!("{}", item.day.format(DATE_FORMAT)),
                &TYPE_OF_WEEK[item.type_of_week as usize],
                format!("{}", item.start.format(TIME_FORMAT)),
                format!("{} {}", item.end.format(TIME_FORMAT), &item.remark),
            );
            writer.encode(row)?;
        }

        Ok(())
    }
}

pub fn generate(conn: &SqliteConnection, id: i32) -> Option<PathBuf> {
    let full_report = FullReport::from_id(conn, id).expect("Failed to load report data");

    full_report
        .write_to_csv()
        .expect("Failed to write CSV file");

    None
}

    render_latex(temp_dir, tex_path)
}
