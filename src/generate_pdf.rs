use std::fs::{self, File};
use std::io::Write;
use std::ops::Add;
use std::path::{Path, PathBuf};
use std::process;

use csv;
use errors::*;
use diesel::SqliteConnection;
use tempdir::TempDir;

use TIME_FORMAT;
use items::{self, InvoiceItem};
use reports::{self, Report};
use weeks::TYPE_OF_WEEK_NAME;

pub struct RawReportData {
    metadata: Report,
    items: Vec<InvoiceItem>,
}

impl RawReportData {
    /// Retrieve all data belonging to a report from the database.
    fn from_id(conn: &SqliteConnection, id: i32) -> Result<RawReportData> {
        let metadata = reports::get(conn, id)?;
        let items = items::get(conn, id)?;
        Ok(RawReportData { metadata, items })
    }

    fn sanitized_path(&self) -> String {
        let slashes_replaced = self.metadata.title.replace('/', "_");
        slashes_replaced.replace('\0', "_")
    }

    fn write_csv(&self) -> Result<()> {
        let mut path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("csv")
            .join(&self.sanitized_path());
        path.set_extension("csv");

        let mut writer = csv::Writer::from_file(path).expect("Failed to create CSV writer");

        for item in &self.items {
            let row = (
                &item.name,
                format!("{}", item.day.format("%d.%m.%y")),
                &TYPE_OF_WEEK_NAME[item.type_of_week as usize],
                format!("{}", item.start.format(TIME_FORMAT)),
                format!("{}", item.end.format(TIME_FORMAT)),
                &item.remark,
            );
            writer
                .encode(&row)
                .chain_err(|| format!("Failed to format CSV row: {:?}", row))?;
        }

        Ok(())
    }

    fn generate_latex(&self) -> String {
        use handlebars::Handlebars;

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

        let path = dir.path().join(self.sanitized_path().add(".tex"));

        let mut file = File::create(&path).expect("Failed to create LaTeX file");
        file.write_all(latex.as_bytes())
            .expect("Failed to write LaTeX to file");

        Some((dir, path))
    }
}

fn render_latex<P: AsRef<Path>>(temp_dir: TempDir, file_path: P) -> Result<PathBuf> {
    // Limit scope of tempdir_path_string so we can close temp_dir later
    {
        let tempdir_path_string = temp_dir.path().to_str().unwrap();
        let file_path_string = file_path.as_ref().to_str().unwrap();

        process::Command::new("xelatex")
            .arg("-output-directory")
            .arg(tempdir_path_string)
            .arg(file_path_string)
            .output()
            .chain_err(|| "Executing XeLaTeX failed")?;
        // TODO handle LaTeX errors
    }

    let pdf = file_path.as_ref().with_extension("pdf");

    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("pdf")
        .join(pdf.file_name().unwrap());

    fs::copy(&pdf, &output_path).expect("Failed to copy PDF file");

    temp_dir
        .close()
        .chain_err(|| "Failed to close temporary directory")?;

    Ok(output_path)
}

pub fn generate(conn: &SqliteConnection, id: i32) -> Result<PathBuf> {
    let full_report = RawReportData::from_id(conn, id)?;

    full_report.write_csv()?;

    let (temp_dir, tex_path) = full_report
        .write_latex()
        .expect("Failed to write LaTeX file");

    let pdf_path = render_latex(temp_dir, tex_path)?;

    reports::set_pdf_generated(conn, id);

    Ok(pdf_path)
}
