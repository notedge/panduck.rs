use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

pub struct SyntectConfig {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl Default for SyntectConfig {
    fn default() -> Self {
        Self { syntax_set: SyntaxSet::load_defaults_newlines(), theme_set: ThemeSet::load_defaults() }
    }
}
