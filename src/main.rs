#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
mod template;
use template::{ Package };

lazy_static! {
    static ref LATEX_ENVIRONMENTS: HashMap<&'static str, &'static str> = {
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

    static ref LATEX_COMMANDS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::with_capacity(4);
        map.insert("\\addplot", "pgfplots");
        map.insert("\\smartdiagram", "smartdiagram");
        map.insert("\\DisplayProof", "bussproofs");
        map.insert("\\tdplotsetmaincoords", "tikz-3dplot");
        return map
    };

    static ref LATEX_COMMANDS_INLINE: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::with_capacity(7);
        map.insert("\\color", "xcolor");
        map.insert("\\textcolor", "xcolor");
        map.insert("\\colorbox", "xcolor");
        map.insert("\\pagecolor", "xcolor");
        return map
    };
}

#[derive(Debug)]
pub struct Formula {
    text: String,
    has_baseline: bool,
}

impl From<&str> for Formula {
    fn from (formula: &str) -> Formula {
        let mut formula_str = String::from(formula);
        let mut is_math_mode: bool = true;
        let mut extra_packages: HashMap<&str, Package> = HashMap::new();

        // Check if there are used certain environments and include corresponding packages
        for (command, env) in LATEX_ENVIRONMENTS.iter() {
            let command_str = format!("\\begin{{{}}}", command);
            let command_asterik_str = format!("\\begin{{{}*}}", command);

            if formula_str.contains(&command_str) || formula_str.contains(&command_asterik_str) {
                is_math_mode = false;
                extra_packages.insert(env, Package {
                    options: vec![],
                    package_str: env.to_string()
                });
            }
        };

        // Check if there are used certain commands and include corresponding packages
        for (command, env) in LATEX_COMMANDS.iter() {
            if formula_str.contains(command) {
                is_math_mode = false;
                extra_packages.insert(env, Package {
                    options: vec![],
                    package_str: env.to_string()
                });
            }
        };

        // Same as above but for inline commands inside math mode
        for (command, env) in LATEX_COMMANDS_INLINE.iter() {
            if formula_str.contains(command) {
                is_math_mode = false;
                extra_packages.insert(env, Package {
                    options: vec![],
                    package_str: env.to_string()
                });
            }
        };

        // Custom rules
        if formula_str.contains("\\xymatrix") || formula_str.contains("\\begin{xy}") {
            extra_packages.insert("xy", Package {
                options: vec!["all"],
                package_str: "xy".to_string(),
            });
        };

        // Other setup
        let is_inline: bool = {
            let inline_matches: Vec<(usize, &str)> = formula.match_indices("\\inline").collect();
            
            if inline_matches.len() == 0 {
                false
            } else {
                let (index, _) = inline_matches[0];
                index == 1
            }
        };

        if is_inline {
            // Replace "\inline" with "\textstyle "
            formula_str = format!("\\textstyle {}", &formula_str[7..]);
        };

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
    let mut st = "\\inline|lol look at this \\begin{xy} shit dude".to_string();
    st = format!("\\textstyle {}", &st[7..]);
    println!("{}", st);
}
