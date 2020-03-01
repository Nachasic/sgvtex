#[macro_use]
extern crate lazy_static;
extern crate tempfile;
extern crate tokio;

mod formula;

use formula::constants::*;

use std::fmt;
use std::error::{ Error };
use std::fs::{ File, DirEntry };
use std::io::{ Write, Read, Seek, SeekFrom, Error as IOError };

use tempfile::{ TempDir, tempdir, tempfile };
use tokio::process::*;

#[derive(Debug)]
pub enum SanitizeError {
    ForbiddenSyntax
}

impl fmt::Display for SanitizeError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SanitizeError::ForbiddenSyntax => write!(formatter, "Notation contains unsupported LaTeX syntax")
        }
    }
}

impl Error for SanitizeError {
    fn description (&self) -> &str {
        match *self {
            SanitizeError::ForbiddenSyntax => "Notation contains unsupported LaTeX syntax",
        }
    }
    fn cause (&self) -> Option<&dyn Error> {
        match *self {
            SanitizeError::ForbiddenSyntax => None
        }
    }
}

use formula::FormulaTemplate;

pub const TEMP_FILE_NAME_LATEX: &'static str = "temp.tex";
pub const TEMP_FILE_NAME_DVI: &'static str = "temp.dvi";
pub const TEMP_FILE_NAME_SVG: &'static str = "temp.svg";

pub type SVG = String;
pub enum RenderingError {
    IOError,
}

pub struct Renderer {
    formula_template: FormulaTemplate,
    temp_dir: TempDir,
    latex_file: File,
    dvi_file: Option<File>,
    svg_file: Option<File>,
}

impl From<FormulaTemplate> for Renderer {
    fn from (template: FormulaTemplate) -> Renderer {
        let temp_dir = tempdir().unwrap();
        let latex_file_path = temp_dir.path().join(TEMP_FILE_NAME_LATEX);

        Renderer {
            formula_template: template,
            temp_dir: temp_dir,
            latex_file: File::create(latex_file_path).unwrap(),
            dvi_file: None,
            svg_file: None,
        }
    }
}

impl Renderer {
    fn write_temp_latex_file (&mut self) -> Result<(), IOError> {
        let file_contents = Vec::from(&self.formula_template.text[..]);

        self.latex_file.write_all(&file_contents[..])
    }

    async fn render_dvi (&mut self) {
        let child = Command::new("latex")
            .arg(self.temp_dir.path().join(TEMP_FILE_NAME_LATEX))
            .arg("2>&1")
            .spawn();
        
        let future = child.expect("Failed to spawn");
        let status = future.await.unwrap();
    }
}

fn main() {
    let st = LATEX_BEGIN_COMMANDS.get("tikzcd").unwrap();
    println!("{}", st);
}
