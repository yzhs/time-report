// Logging
#[macro_use]
extern crate log;
extern crate colog;

// Parse the data from a CSV file
extern crate csv;

// Compute duration based on start and end time
extern crate time;

// Argument parsing
extern crate argonaut;


use std::env;
use std::path::Path;
use std::process;

use argonaut::{ArgDef, help_arg, version_arg};


const NUM_WORKERS: usize = 100;

const TABLE_HEADER: [&str; 5] = ["Name", "Datum", "von", "bis", "Woche/Bemerkung"];


/// Represents a row in the table containing the date a particular person worked on, how many hours
/// and minutes they worked, and a free-text comment filed.
///
/// Each of these structures is a row in the generated table, but not every row in the table
/// represents a Line struct.
#[derive(Debug)]
struct Line {
    date: String, // TODO use a proper datatype (time::something?)
    hours: u16,
    minutes: u16,
    remark: String,
}

/// Data related to one person:  Their name, how long they worked in the accounting period and the
/// specific data of when the worked.  Each of these structures is rendered as one table.
#[derive(Debug)]
struct Worker {
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

    pub fn to_latex(&self) -> String {
        let mut result = String::with_capacity(500);

        result.push_str("  \\begin{person}{");
        result.push_str(&self.name);
        result.push_str("}\n");

        for line in &self.lines {
            result.push_str(&line.to_latex());
        }

        result.push_str("  \\end{person}");

        result
    }
}

impl Line {
    pub fn to_latex(&self) -> String {
        format!("    & {} & {} & {} & {}\\\\\n",
                self.date.replace(".", ".\\,"), self.hours, self.minutes, self.remark)
    }
}



/// Read a given CSV file a list of `Line`s.
#[allow(unreachable_code)] // TODO better solution?
fn read_csv_file<P: AsRef<Path>>(path: P) -> csv::Result<Vec<Worker>> {
    let mut reader = csv::Reader::from_file(path)
        ?
        .has_headers(true)
        .flexible(true);

    let headers = try!(reader.headers());
    let rows = if headers == TABLE_HEADER {
        info!("Using the automatic duration column set in default order");

        // name, date, start, end, remark
        type Row = (String, String, String, String, Vec<String>);
        let row_to_line = |row: Row| {
            let start = time::strptime(&row.2, "%H:%M:00").expect("Invalid start time");
            let end = time::strptime(&row.3, "%H:%M:00").expect("Invalid end time");
            let duration = end - start;
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            if hours > 4 {
                warn!("Hours > 4 in line")
            }
            (row.0,
             Line {
                 date: row.1,
                 hours: hours as u16,
                 minutes: minutes as u16,
                 remark: row.4.join(" "),
             })
        };

        let rows: Vec<Row> = try!(reader.decode().collect());
        rows.into_iter().map(row_to_line).collect()
    } else if headers == ["Name", "Datum", "Stunden", "Minuten", "Woche/Bemerkung"] {
        info!("Using the manual duration column set in default order");

        // name, date, hours, minutes, remark
        type RowManual = (String, String, u16, u16, Vec<String>);
        let row_to_line = |row: RowManual| {
            (row.0,
             Line {
                 date: row.1,
                 hours: row.2 as u16,
                 minutes: row.3 as u16,
                 remark: row.4.join(" "),
             })
        };

        let rows: Vec<RowManual> = try!(reader.decode().collect());
        rows.into_iter().map(row_to_line).collect()
    } else {
        panic!("Invalid headers: {:?}", headers);
        vec![]
    };

    // Collect the data for each of the workers
    use std::collections::HashMap;
    let mut workers = HashMap::with_capacity(NUM_WORKERS);
    for (name, line) in rows {
        let mut person = workers.entry(name.clone()).or_insert(Worker::new(name));
        person.hours += line.hours as u32;
        person.minutes += line.minutes as u32;
        person.lines.push(line);
    }

    // Produce a list of workers
    let result = workers.into_iter().map(|(_, person)| person).collect();

    Ok(result)
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    colog::init();
    let mut csv_file = String::new();

    {
        let foo = vec![
            ArgDef::positional("csv-file", &mut csv_file).help("The CSV file containing the data."),
            help_arg("Compile a PDF from the data contained in the given CSV file.").short("h"),
            version_arg(),
        ];

        match argonaut::parse("generate-pdf", &args, foo) {
            Ok(_error_code) => {}
            //Err(ParseError::Interrupted(_)) => {
            //    process::exit(-1);
            //},
            Err(_) => {
                process::exit(1);
            }
        };
    }

    let data = read_csv_file(csv_file).unwrap();
    for worker in data {
        println!("{}", worker.to_latex());
    }
}
