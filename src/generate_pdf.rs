use std::fs::File;
use std::io::Write;
use std::ops::Add;
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
pub struct Employee {
    name: String, // TODO separate last name?
    hours: u32,
    minutes: u32,
    lines: Vec<Line>,
}

impl Employee {
    pub fn new(name: String) -> Employee {
        Employee {
            name,
            hours: 0,
            minutes: 0,
            lines: vec![],
        }
    }

    // XXX This assumes that the last sequence of non-space characters is the family name (or
    // something similar). This works pretty well for German names, but fails for e.g. Spanish
    // names where a person has the one of the last names of each of their parents.
    //
    // It should, however, be good enough for our purposes, which is just to sort a list of
    // employees by name. Should more accurate sorting be necessary, we could either provide
    // separate fields for first and last names (which only works for people who *have* a last
    // name), or we could use Calibre's approach: In addition to the name, store a version of the
    // name used for sorting.
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

pub struct RawReportData {
    metadata: Report,
    items: Vec<InvoiceItem>,
}

impl RawReportData {
    fn from_id(conn: &SqliteConnection, id: i32) -> Result<RawReportData, ()> {
        let metadata = reports::get(conn, id).ok_or(())?;
        let items = vec![];
        Ok(RawReportData { metadata, items })
    }

    fn write_csv(&self) -> csv::Result<()> {
        const TYPE_OF_WEEK: [&str; 4] = ["A", "B", "C", "D"];

        let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("csv")
            .join(&self.metadata.title);
        path.set_extension("csv");

        let mut writer = csv::Writer::from_file(path).expect("Failed to create CSV writer");

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

    fn generate_latex(&self) -> String {
        use handlebars::{Handlebars, RenderContext, RenderError};

        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("latex", include_str!("template.tex.hbs"))
            .expect("Failed to register template");

        // TODO When updating to Handlebars 1, uncomment this:
        //handlebars.set_strict_mode(true);

        let params = reports::PerEmployeeReport::generate(&::db::connect(), self.metadata.id);

        handlebars
            .render("latex", &params)
            .expect("Failed to render template")
    }

    fn write_latex(&self) -> Option<(TempDir, PathBuf)> {
        let latex = self.generate_latex();

        let dir = TempDir::new("generate-pdf").expect("Failed to create temporary directory");

        let path = dir.path().join(self.metadata.title.clone().add(".tex"));

        let mut file = File::create(&path).expect("Failed to create LaTeX file");
        file.write_all(latex.as_bytes())
            .expect("Failed to write LaTeX to file");

        Some((dir, path))
    }
}

fn render_latex<P: AsRef<Path>>(temp_dir: TempDir, file_path: P) -> Option<PathBuf> {
    // Limit scope of tempdir_path_string so we can close temp_dir later
    {
        let tempdir_path_string = temp_dir.path().to_str().unwrap();
        let file_path_string = file_path.as_ref().to_str().unwrap();
        process::Command::new("xelatex")
            .arg("-output-directory")
            .arg(tempdir_path_string)
            .arg(file_path_string)
            .output()
            .expect("Executing LaTeX failed");
        // TODO handle LaTeX errors
    }

    let pdf = file_path.as_ref().with_extension("pdf");
    // TODO figure out where to put the generated PDF
    //fs::copy(pdf, file_path.with_extension("pdf")).expect("Failed to copy PDF file");

    temp_dir
        .close()
        .expect("Failed to close temporary directory");

    Some(pdf)
}

pub fn generate(conn: &SqliteConnection, id: i32) -> Option<PathBuf> {
    let full_report = RawReportData::from_id(conn, id).expect("Failed to load report data");

    full_report.write_csv().expect("Failed to write CSV file");

    let (temp_dir, tex_path) = full_report
        .write_latex()
        .expect("Failed to write LaTeX file");

    render_latex(temp_dir, tex_path)
}
