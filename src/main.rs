#[macro_use]
extern crate lazy_static;
extern crate handlebars;
extern crate serde;
#[macro_use]
extern crate serde_json;

use handlebars::{ Handlebars };

use std::collections::HashMap;
use std::fmt;
use std::error::{ Error };
use std::fs::{ read_to_string };

mod templating;
use templating::{ Package };

mod constants;
use constants::*;

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

#[derive(Debug)]
pub struct FormulaTemplate {
    text: String,
    has_baseline: bool,
    is_math_mode: bool,
    packages: Vec<String>
}

impl FormulaTemplate {
    fn apply_base_template (&mut self) {
        let source_template = match self.is_math_mode {
            true => read_to_string(&DISPLAY_FORMULA_TPL_PATH).unwrap(),
            false => read_to_string(&COMMON_FORMULA_TPL_PATH).unwrap()
        };
        let handlebars = Handlebars::new();

        self.text = handlebars.render_template(&source_template, &json!({ "formula": self.text })).unwrap();
    }

    fn apply_document_template (&mut self) {
        let source_template = read_to_string(&DOCUMENT_TPL_PATH).unwrap();
        let handlebars = Handlebars::new();

        self.text = handlebars.render_template(&source_template, &json!({ "formula": self.text, "extra_package_codes": self.packages })).unwrap();
    }
}

impl From<&str> for FormulaTemplate {
    fn from<'a> (formula: &str) -> FormulaTemplate {
        let mut formula_str = String::from(formula);
        let mut is_math_mode: bool = true;
        let mut extra_packages: HashMap<&str, Package> = HashMap::new();

        let mut should_update_packages_for_command = |command: &str, env: &'a str, options: Vec<&'a str>| {
            let command_is_found = formula_str.contains(command);
            if command_is_found {
                extra_packages.insert(env, Package {
                    options: options,
                    package_str: env.to_string(),
                });
            };
            command_is_found
        };

        // Check if there are used certain environments and include corresponding packages
        for (command, env) in LATEX_BEGIN_COMMANDS.iter() {
            if should_update_packages_for_command(&format!("\\begin{{{}}}", command), env, vec![])
            || should_update_packages_for_command(&format!("\\begin{{{}*}}", command), env, vec![]) {
                is_math_mode = false;
            };
        };

        // Check if there are used certain commands and include corresponding packages
        for (command, env) in LATEX_COMMANDS.iter() {
            if should_update_packages_for_command(command, env, vec![]) {
                is_math_mode = false;
            };
        };

        // Same as above but for inline commands inside math mode
        for (command, env) in LATEX_COMMANDS_INLINE.iter() {
            if should_update_packages_for_command(command, env, vec![]) {
                is_math_mode = false;
            };
        };

        // Custom rules
        should_update_packages_for_command("\\xymatrix", "xy", vec!["all"]);
        should_update_packages_for_command("\\begin{xy}", "xy", vec!["all"]);

        // Other setup
        let is_inline: bool = {
            let inline_matches: Vec<(usize, &str)> = formula.match_indices(LATEX_INLINE_COMMAND).collect();
            
            if inline_matches.len() == 0 {
                false
            } else {
                let (index, _) = inline_matches[0];
                index == 1
            }
        };

        if is_inline {
            // Replace "\inline" with "\textstyle "
            formula_str = format!("\\textstyle {}", &formula_str[LATEX_INLINE_COMMAND.len()..]);
        };

        let mut template = FormulaTemplate {
            has_baseline: false,
            is_math_mode: is_math_mode,
            text: formula_str,
            packages: {
                let mut vec: Vec<String> = vec![];
                for (_, package) in extra_packages.iter() {
                    let package_insert_str = package.get_code();
                    vec.push(package_insert_str)
                };
                vec
            }
        };

        template.apply_base_template();
        template.apply_document_template();
        template
    }
}

// TODO: document template tests with dummy data
// TODO: snapshot documetn tests with real formulas

#[test]
fn inline_formula_test () {
    let mut template = FormulaTemplate {
        text: read_to_string(&"./fixtures/inline_formula.tex").unwrap(),
        is_math_mode: false,
        has_baseline: false,
        packages: vec![],
    };
    let expected_result = read_to_string(&"./fixtures/expected/common.tex").unwrap();

    template.apply_base_template();
    assert_eq!(template.text, expected_result);
}

#[test]
fn block_formula_test () {
    let mut template = FormulaTemplate {
        text: read_to_string(&"./fixtures/block_formula.tex").unwrap(),
        is_math_mode: true,
        has_baseline: false,
        packages: vec![],
    };
    let expected_result = read_to_string(&"./fixtures/expected/display_formula.tex").unwrap();

    template.apply_base_template();
    assert_eq!(template.text, expected_result);
}

fn main() {
    let st = LATEX_BEGIN_COMMANDS.get("tikzcd").unwrap();
    println!("{}", st);
}
