#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fmt;
use std::error::{ Error };
mod templating;
use templating::{ Package };

type LaTeXEnvironmentName = &'static str;
type LaTeXCommand = &'static str;
type LaTeXPackageName = &'static str;

lazy_static! {
    static ref LATEX_BEGIN_COMMANDS: HashMap<LaTeXEnvironmentName, LaTeXPackageName> = {
        let mut map = HashMap::with_capacity(7);
        map.insert("eqnarray", "eqnarray");
        map.insert("tikzcd", "tikz-cd");
        map.insert("tikzpicture", "tikz");
        map.insert("circuitikz", "circuitikz");
        map.insert("sequencediagram", "pgf-umlsd");
        map.insert("prooftree", "bussproofs");
        map.insert("align", "");
        return map
    };

    static ref LATEX_COMMANDS: HashMap<LaTeXCommand, LaTeXPackageName> = {
        let mut map = HashMap::with_capacity(4);
        map.insert("\\addplot", "pgfplots");
        map.insert("\\smartdiagram", "smartdiagram");
        map.insert("\\DisplayProof", "bussproofs");
        map.insert("\\tdplotsetmaincoords", "tikz-3dplot");
        return map
    };

    static ref LATEX_COMMANDS_INLINE: HashMap<LaTeXCommand, LaTeXPackageName> = {
        let mut map = HashMap::with_capacity(7);
        map.insert("\\color", "xcolor");
        map.insert("\\textcolor", "xcolor");
        map.insert("\\colorbox", "xcolor");
        map.insert("\\pagecolor", "xcolor");
        return map
    };
}
const LATEX_INLINE_COMMAND: &'static str = "\\inline";

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
}

impl From<&str> for FormulaTemplate {
    fn from (formula: &str) -> FormulaTemplate {
        let mut formula_str = String::from(formula);
        let mut is_math_mode: bool = true;
        let mut extra_packages: HashMap<&str, Package> = HashMap::new();

        let should_update_packages_for_command = |command: &str, env: &str, options: Vec<&str>| {
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

        // let formula_template_name = if is_math_mode { "displayformula" } else { "common" };
		// $tpl = $isMathMode ? 'displayformula' : 'common';

		// ob_start();
		// include $this->dir . $tpl . '.php';
		// $documentContent = ob_get_clean();

		// ob_start();
		// include $this->dir . 'document.php';
		// $text = ob_get_clean();

		// return new Formula($text, $isMathMode);
    }
}

fn main() {
    let st = LATEX_BEGIN_COMMANDS.get("tikzcd").unwrap();
    println!("{}", st);
}
