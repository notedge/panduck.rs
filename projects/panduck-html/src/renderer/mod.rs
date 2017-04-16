pub use self::{plain_text::PlainHTML, pretty_print::PrettyHTML};
use notedown_ast::{
    nodes::{CodeNode, Header, ListView, MathKind, MathNode, StyleKind, TableView, TextNode},
    ASTKind, ASTNode, ASTNodes, Result,
};
use std::{
    fmt,
    fmt::{Arguments, Write},
};

mod plain_text;
mod pretty_print;

pub struct HTMLRenderer {
    xhtml: bool,
    max_width: usize,
    math_renderer: Option<fn(&MathNode) -> String>,
    code_renderer: Option<fn(&CodeNode) -> String>,
    buffer: String,
}

impl Default for HTMLRenderer {
    fn default() -> Self {
        Self { xhtml: false, max_width: 144, math_renderer: None, code_renderer: None, buffer: String::new() }
    }
}

impl Write for HTMLRenderer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buffer.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.buffer.write_char(c)
    }

    fn write_fmt(self: &mut Self, args: Arguments<'_>) -> fmt::Result {
        self.buffer.write_fmt(args)
    }
}

impl HTMLRenderer {
    pub fn render_plain(&mut self, node: &ASTNode) -> Result<String> {
        self.buffer.clear();
        node.plain_html(self)?;
        Ok(self.buffer.to_owned())
    }
    pub fn render_pretty(&mut self, node: &ASTNode) -> Result<String> {
        self.buffer.clear();
        node.pretty_html(self).render_fmt(self.max_width, &mut self.buffer)?;
        Ok(self.buffer.to_owned())
    }
}

impl HTMLRenderer {
    pub fn set_math_renderer(mut self, renderer: fn(&MathNode) -> String) -> Self {
        self.math_renderer = Some(renderer);
        return self;
    }
    pub fn set_code_renderer(mut self, renderer: fn(&CodeNode) -> String) -> Self {
        self.code_renderer = Some(renderer);
        return self;
    }
}
