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

fn main() {
    let st = LATEX_BEGIN_COMMANDS.get("tikzcd").unwrap();
    println!("{}", st);
}
