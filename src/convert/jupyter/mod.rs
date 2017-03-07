use crate::parse_markdown;
use anyhow::{Error, Result};
use notedown_ast::AST;
use serde_json::{Map, Value};

pub fn jupyter_from_json(root: &Value) -> Result<AST> {
    match root {
        Value::Object(o) => Ok(jupyter_root(o)),
        _ => Err(Error::msg("Not a valid jupyter json")),
    }
}

fn jupyter_root(dict: &Map<String, Value>) -> AST {
    if let Some(cells) = dict.get("cells") {
        if let Value::Array(v) = cells {
            return jupyter_cells(v);
        }
    }
    AST::Statements(vec![])
}

fn jupyter_cells(cells: &Vec<Value>) -> AST {
    let mut out = vec![];
    for cell in cells {
        if let Value::Object(o) = cell {
            out.extend(jupyter_cell(o))
        }
    }
    AST::Statements(out)
}

fn jupyter_cell(dict: &Map<String, Value>) -> Vec<AST> {
    let mut cell_type = "markdown";
    if let Some(key) = dict.get("cell_type") {
        if let Value::String(s) = key {
            cell_type = s
        }
    };
    match cell_type {
        "markdown" => jupyter_markdown(dict.get("source").unwrap()),
        _ => vec![],
    }
}

fn jupyter_markdown(dict: &Value) -> Vec<AST> {
    let lines: Vec<String> = match dict {
        Value::Array(v) => v.into_iter().map(|v| jupyter_string(v)).collect(),
        _ => vec![],
    };
    if let Ok(o) = parse_markdown(&lines.join("\n")) {
        if let AST::Statements(v) = o {
            return v;
        }
    }
    vec![]
}

fn jupyter_string(o: &Value) -> String {
    match o {
        Value::String(v) => v.to_owned(),
        _ => String::new(),
    }
}
