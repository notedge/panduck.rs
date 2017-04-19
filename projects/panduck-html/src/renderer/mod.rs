use notedown_ast::{
    nodes::{CodeNode, Header, ListView, MathKind, MathNode, StyleKind, TableView, TextNode},
    ASTKind, ASTNode, ASTNodes, Result,
};
use pretty::RcDoc;
use std::{
    fmt,
    fmt::{Arguments, Write},
};

mod nodes;
mod text;

pub trait PrettyHTML {
    fn pretty_html(&self, f: &mut PrettyRenderer) -> RcDoc<()>;
}

pub struct PrettyRenderer {
    xhtml: bool,
    max_width: usize,
    math_renderer: Option<fn(&MathNode) -> String>,
    code_renderer: Option<fn(&CodeNode) -> String>,
    buffer: String,
}

impl Default for PrettyRenderer {
    fn default() -> Self {
        Self { xhtml: false, max_width: 144, math_renderer: None, code_renderer: None, buffer: String::new() }
    }
}

impl Write for PrettyRenderer {
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

impl PrettyRenderer {
    pub fn render_pretty(&mut self, node: &ASTNode) -> Result<String> {
        self.buffer.clear();
        node.pretty_html(self).render_fmt(self.max_width, &mut self.buffer)?;
        Ok(self.buffer.to_owned())
    }
}

impl PrettyRenderer {
    pub fn set_math_renderer(mut self, renderer: fn(&MathNode) -> String) -> Self {
        self.math_renderer = Some(renderer);
        return self;
    }
    pub fn set_code_renderer(mut self, renderer: fn(&CodeNode) -> String) -> Self {
        self.code_renderer = Some(renderer);
        return self;
    }
}
