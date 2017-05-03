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

// Create a secure temporary directory to handle the LaTeX side of things.
extern crate tempdir;


use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process;

use argonaut::{ArgDef, help_arg, version_arg};
use tempdir::TempDir;


const NUM_WORKERS: usize = 100;

const TABLE_HEADER: [&str; 5] = ["Name", "Datum", "von", "bis", "Woche/Bemerkung"];


/// Represents a row in the table containing the date a particular person worked on, how many hours
/// and minutes they worked, and a free-text comment filed.
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

    pub fn to_latex(&self) -> String {
        let mut result = String::with_capacity(500);

        result.push_str("  \\begin{person}{");
        result.push_str(&self.name);
        result.push_str("}\n");

        for line in &self.lines {
            result.push_str(&line.to_latex());
        }

        result.push_str(&format!("    \\midrule\n    \\bfseries{{Summe}} && \\bfseries{{{}}} & \\bfseries{{{}}}\\\\\n",
                                self.hours,
                                self.minutes));
        result.push_str("  \\end{person}\n\n");

        result
    }

    pub fn last_name(&self) -> String {
        let name = &self.name;
        if name.contains(",") {
                name.split(",").next()
            } else {
                name.split(" ").last()
            }
            .unwrap()
            .to_string()
    }
}

impl Line {
    pub fn to_latex(&self) -> String {
        format!("    & {} & {} & {} & {}\\\\\n",
                self.date.replace(".", ".\\,"), self.hours, self.minutes, self.remark)
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
            warn!("Remark field does not separate week from comment: {:?}", self);
        }

        // TODO check date
        //if self.date > now {
        //    error!("Date is in the future: {}", self.date);
        //} else if now - self.date > Duration::years(1) {
        //    error!("Date is more than one year in the past: {}", self.date);
        //} else if self.date.tm_wday == 0 || self.date.tm_wday == 6 {
        //    error!("Date is during a weekend: {}", self.date);
        //} else if self.date.is_known_holiday() {
        //    error!("Date is on a holiday: {}", self.date);
        //}
    }
}

fn check_start_and_end(start: &time::Tm, end: &time::Tm) {
    // TODO figure out how to report errors with better context
    if start.tm_hour < 12 {
        warn!("Start time before noon: {:?}", start);
    }
    if end.tm_hour > 16 {
        warn!("End time after 4 o'clock: {:?}", end);
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
            check_start_and_end(&start, &end);

            let duration = end - start;
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;

            let line = Line {
                date: row.1,
                hours: hours as u16,
                minutes: minutes as u16,
                remark: row.4.join(" "),
            };
            line.check_data();
            (row.0, line)
        };

        let rows: Vec<Row> = try!(reader.decode().collect());
        rows.into_iter().map(row_to_line).collect()
    } else if headers == ["Name", "Datum", "Stunden", "Minuten", "Woche/Bemerkung"] {
        info!("Using the manual duration column set in default order");

        // name, date, hours, minutes, remark
        type RowManual = (String, String, u16, u16, Vec<String>);
        let row_to_line = |row: RowManual| {
            let line = Line {
                date: row.1,
                hours: row.2 as u16,
                minutes: row.3 as u16,
                remark: row.4.join(" "),
            };
            line.check_data();
            (row.0, line)
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

    // Produce a list of workers and handle carry
    let mut result: Vec<_> = workers
        .into_iter()
        .map(|(_, mut person)| {
                 person.hours += person.minutes / 60;
                 person.minutes %= 60;
                 person
             })
        .collect();

    // Sort the list by name
    result.sort_by_key(|k| k.last_name());

    Ok(result)
}

const LATEX_HEADER_1: &[u8] = b"\\documentclass[a4paper]{article}
\\usepackage[ngerman]{babel}
\\usepackage{booktabs} % Nicer vertical lines
\\usepackage{tabularx} % paragraphs in tables
\\usepackage{fullpage} % Use the entire page
\\usepackage{fontspec} % Allow the use of unicode
\\usepackage{array} % Provide b column type for bottom alignment

%\\renewcommand{\\arraystretch}{1.2}

\\newcommand*{\\headerfor}[1]{%
  \\bfseries{#1} & \\textsc{Datum} & \\textsc{Stunden} & \\textsc{Minuten} &
  \\textsc{Woche/Bemerkung}\\\\
  \\midrule
}
\\newenvironment{person}[1]{%
  \\begin{tabular*}{0.96\\linewidth}{b{0.3\\textwidth}rrrp{0.28\\textwidth}}
    \\headerfor{#1}
}{%
  \\end{tabular*}\\vspace{1cm}
}

\\title{Abrechnung BetreuerInnen ";

const LATEX_HEADER_2: &[u8] = b"\\vspace{-1cm}}
\\author{}
\\date{\\today}

\\begin{document}
\\maketitle
";

const LATEX_FOOTER: &[u8] = b"\\end{document}\n";

/// Create a PDF file from the given workers' data.
pub fn generate_pdf<P: AsRef<Path>>(input: P, workers: &[Worker]) -> Result<(), io::Error> {
    let input = input.as_ref();
    let title = input.file_stem().unwrap().to_str().unwrap();

    let dir = TempDir::new("generate-pdf")?;

    // Generate LaTeX file
    let file_path = dir.path().join("abrechnung.tex");
    let mut f = File::create(&file_path)?;

    // Write LaTeX code
    f.write_all(LATEX_HEADER_1)?;
    write!(f, "{}", title)?;
    f.write_all(LATEX_HEADER_2)?;

    for worker in workers {
        write!(f, "{}", worker.to_latex())?;
    }

    f.write_all(LATEX_FOOTER)?;

    // and sync to disc
    f.sync_all()?;
    drop(f);

    // Run XeLaTeX
    {
        let tempdir_path_string = dir.path().to_str().unwrap();
        let file_path_string = file_path.to_str().unwrap();
        process::Command::new("xelatex")
            .arg("-output-directory")
            .arg(tempdir_path_string)
            .arg(file_path_string)
            .output().unwrap();
        // TODO handle LaTeX errors
    }

    let pdf = file_path.with_extension("pdf");
    std::fs::copy(pdf, input.with_extension("pdf"))?;

    dir.close()?;

    Ok(())
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

    let workers = read_csv_file(&csv_file).unwrap();
    generate_pdf(csv_file, &workers).unwrap();
}
