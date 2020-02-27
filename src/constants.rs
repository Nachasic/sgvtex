use std::collections::HashMap;

pub type LaTeXEnvironmentName = &'static str;
pub type LaTeXCommand = &'static str;
pub type LaTeXPackageName = &'static str;

lazy_static! {
    pub static ref LATEX_BEGIN_COMMANDS: HashMap<LaTeXEnvironmentName, LaTeXPackageName> = {
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

    pub static ref LATEX_COMMANDS: HashMap<LaTeXCommand, LaTeXPackageName> = {
        let mut map = HashMap::with_capacity(4);
        map.insert("\\addplot", "pgfplots");
        map.insert("\\smartdiagram", "smartdiagram");
        map.insert("\\DisplayProof", "bussproofs");
        map.insert("\\tdplotsetmaincoords", "tikz-3dplot");
        return map
    };

    pub static ref LATEX_COMMANDS_INLINE: HashMap<LaTeXCommand, LaTeXPackageName> = {
        let mut map = HashMap::with_capacity(7);
        map.insert("\\color", "xcolor");
        map.insert("\\textcolor", "xcolor");
        map.insert("\\colorbox", "xcolor");
        map.insert("\\pagecolor", "xcolor");
        return map
    };
}
pub const LATEX_INLINE_COMMAND: &'static str = "\\inline";
pub const TPL_DISPLAY_FORMULA_PATH: &'static str = "./tpl/display_formula.tex";
pub const TPL_COMMON_FORMULA_PATH: &'static str = "./tpl/common.tex";
pub const TPL_DOCUMENT_PATH: &'static str = "./tpl/document.tex";
pub const TPL_TOKEN_FORMULA: &'static str = r"~FORMULA~";
pub const TPL_TOKEN_DOCUMENT_CONTENT: &'static str = r"~DOCUMENT_CONTENT~";
pub const TPL_TOKEN_DOCUMENT_PACKAGE_CODES: &'static str = r"~PACKAGE_CODES~";
