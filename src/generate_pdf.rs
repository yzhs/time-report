use std::fs::{self, File};
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

pub const TYPE_OF_WEEK: [&str; 4] = ["A", "B", "C", "D"];

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

    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pdf")
        .join(pdf.file_name().unwrap());

    fs::copy(&pdf, &output_path).expect("Failed to copy PDF file");

    temp_dir
        .close()
        .expect("Failed to close temporary directory");

    Some(output_path)
}

pub fn generate(conn: &SqliteConnection, id: i32) -> Option<PathBuf> {
    let full_report = RawReportData::from_id(conn, id).expect("Failed to load report data");

    full_report.write_csv().expect("Failed to write CSV file");

    let (temp_dir, tex_path) = full_report
        .write_latex()
        .expect("Failed to write LaTeX file");

    let pdf_path = render_latex(temp_dir, tex_path);

    reports::set_pdf_generated(conn, id);

    pdf_path
}
