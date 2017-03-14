///
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub indent_use_space: bool,
    pub indent_size: u8,
    ///
    /// `text{ F } `
    pub inline_space: u8,
    ///
    pub multiline_newline: bool,
    ///
    pub attribute_newline: bool,
    ///
    pub align_attribute: bool,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_use_space: true,
            indent_size: 4,
            inline_space: 1,
            multiline_newline: true,
            attribute_newline: true,
            align_attribute: true,
        }
    }
}
