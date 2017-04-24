use notedown_ast::{nodes::CodeNode, Result};
use syntect::{
    highlighting::{Theme, ThemeSet},
    html::highlighted_html_for_string,
    parsing::SyntaxSet,
};

pub struct SyntectConfig {
    syntax_set: SyntaxSet,
    theme: Theme,
}

impl Default for SyntectConfig {
    fn default() -> Self {
        Self {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            // base16-eighties.dark
            // base16-mocha.dark
            // base16-ocean.dark
            // base16-ocean.light
            // InspiredGitHub
            // Solarized (dark)
            // Solarized (light)
            theme: ThemeSet::load_defaults().themes["InspiredGitHub"].to_owned(),
        }
    }
}

impl SyntectConfig {
    pub fn render_html(&self, code: &CodeNode) -> Result<String> {
        let ss = &self.syntax_set;
        let sr = ss.find_syntax_by_extension(&code.language).unwrap_or(ss.find_syntax_plain_text());
        let html = highlighted_html_for_string(&code.code, ss, sr, &self.theme);
        return Ok(html);
    }
}
