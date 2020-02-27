#[macro_use]
extern crate lazy_static;

mod formula;

use formula::*;
use formula::constants::*;
use std::fmt;
use std::error::{ Error };
use std::fs::{ read_to_string };

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


#[test]
fn document_formula () {
    let template = FormulaTemplate::from(
        &read_to_string(&"./fixtures/inline_formula.tex").unwrap()[..]
    );
    let expected = read_to_string(&"./fixtures/expected/document_formula.tex").unwrap();

    assert_eq!(template.text, expected);
}

#[test]
fn document_tikz () {
    let expected = read_to_string(&"./fixtures/expected/document_tikz.tex").unwrap();
    let template = FormulaTemplate::from(
        &read_to_string(&"./fixtures/tikz.tex").unwrap()[..]
    );

    assert_eq!(template.text, expected);
}

fn main() {
    let st = LATEX_BEGIN_COMMANDS.get("tikzcd").unwrap();
    println!("{}", st);
}
