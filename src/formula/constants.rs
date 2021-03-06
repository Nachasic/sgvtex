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

// Main LaTeX document - final output template
pub const TPL_DOCUMENT_BEGIN: &'static str = "./tpl/document/begin.tex";
pub const TPL_DOCUMENT_BODY: &'static str = "./tpl/document/body.tex";
pub const TPL_DOCUMENT_END: &'static str = "./tpl/document/end.tex";

// Display formula template
pub const TPL_DISPLAY_FORMULA_BEGIN: &'static str = "./tpl/display/begin.tex";
pub const TPL_DISPLAY_FORMULA_END: &'static str = "./tpl/display/end.tex";

// Common formula template
pub const TPL_COMMON_FORMULA_BEGIN: &'static str = "./tpl/common/begin.tex";
pub const TPL_COMMON_FORMULA_END: &'static str = "./tpl/common/end.tex";
