mod table_view;
mod writers;

use crate::{nodes::Header, ASTKind, ASTNode, ASTNodes, CodeNode, ListView, MathKind, MathNode, StyleKind, TextNode, TableView};
use std::{
    fmt,
    fmt::{Arguments, Write},
};

pub trait WriteHTML {
    fn write_html(&self, f: &mut HTMLRenderer) -> fmt::Result;
}

pub struct HTMLRenderer {
    math_renderer: Option<fn(&MathNode) -> String>,
    code_renderer: Option<fn(&CodeNode) -> String>,
    buffer: String,
}

impl Default for HTMLRenderer {
    fn default() -> Self {
        Self { math_renderer: None, code_renderer: None, buffer: String::new() }
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
    pub fn render(&mut self, node: &ASTNode) -> String {
        self.buffer.clear();
        node.write_html(self);
        return self.buffer.to_owned();
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
