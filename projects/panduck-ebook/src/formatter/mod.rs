mod pretty_print;

pub use self::pretty_print::PrettyPrint;
use crate::FormatterConfig;
use std::{
    fmt::{self, Write},
    mem::swap,
};

#[derive(Debug, Clone)]
pub struct FluentFormatter {
    config: FormatterConfig,
    indent: String,
    buffer: String,
}

impl Default for FluentFormatter {
    fn default() -> Self {
        Self { config: FormatterConfig::default(), indent: String::new(), buffer: String::new() }
    }
}

impl Write for FluentFormatter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        write!(self.buffer, "{}", s)
    }
}

impl FluentFormatter {
    pub fn get_buffer(&self) -> &str {
        self.buffer.as_str()
    }
    pub fn swap_indent(&mut self, rhs: &mut String) {
        swap(&mut self.indent, rhs)
    }
    pub fn write_new_indent<T: PrettyPrint>(&mut self, body: &T, indent: &str) -> fmt::Result {
        let mut indent = self.indent.to_owned() + indent;
        self.swap_indent(&mut indent);
        body.pretty_print(self)?;
        self.swap_indent(&mut indent);
        Ok(())
    }

    pub fn write_with_indent(&mut self, line: &str) {
        self.buffer.push_str(&self.indent);
        self.buffer.push_str(line);
    }
    pub fn write_new_line(&mut self) {
        self.buffer.push('\n')
    }
    pub fn tab(&self) -> String {
        match self.config.indent_use_space {
            true => " ".repeat(self.config.indent_size as usize),
            false => String::from("\t"),
        }
    }
    pub fn clear(&mut self) {
        self.indent.clear();
        self.buffer.clear();
    }
    pub fn pretty_print(&mut self, text: impl PrettyPrint) -> Result<&str, fmt::Error> {
        self.clear();
        text.pretty_print(self)?;
        Ok(self.get_buffer())
    }
}
