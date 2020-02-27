use std::fs::{ read_to_string };

mod package;
use package::*;

pub mod constants;
use constants::*;

use std::collections::HashMap;

#[derive(Debug)]
pub struct FormulaTemplate {
    pub text: String,
    has_baseline: bool,
    is_math_mode: bool,
    packages: Vec<String>
}

impl FormulaTemplate {
    fn constuct_document (&mut self) {
        // TODO: formula validation
        let document_begin = &read_to_string(&TPL_DOCUMENT_BEGIN).unwrap()[..];
        let document_body = &read_to_string(&TPL_DOCUMENT_BODY).unwrap()[..];
        let document_end = &read_to_string(&TPL_DOCUMENT_END).unwrap()[..];
        let packages_str = &self.packages.join("\n")[..];
        let formula_str = &self.text[..];

        let (formula_begin, formula_end) = match self.is_math_mode {
            true => (
                read_to_string(&TPL_DISPLAY_FORMULA_BEGIN).unwrap(),
                read_to_string(&TPL_DISPLAY_FORMULA_END).unwrap()
            ),
            false => (
                read_to_string(&TPL_COMMON_FORMULA_BEGIN).unwrap(),
                read_to_string(&TPL_COMMON_FORMULA_END).unwrap()
            )
        };

        self.text = [
            document_begin,
                packages_str,
                document_body,
                &formula_begin[..],
                    formula_str,
                &formula_end[..],
            document_end
        ].join("\n").to_string();
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

        template.constuct_document();
        template
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