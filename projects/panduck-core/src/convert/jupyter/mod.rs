use super::*;
use crate::{error::Error::ParseError, parse_markdown, Result};
use notedown_parser::ASTKind;
use serde_json::{Map, Value};

pub fn register_jupyter(r: &mut ExtensionRegistrar) {
    let ext = vec!["note"];
    let parser = |input| Ok(ParserConfig::default().parse(input)?.to_notedown());
    let new = ExtensionHandler { try_extension: BTreeSet::from_iter(ext.iter().map(String::from)), parser };
    r.insert(new)
}

pub fn parse_jupyter(text: &str) -> Result<AST> {
    let v: Value = serde_json::from_str(text)?;
    Ok(jupyter_from_json(&v)?)
}

pub fn jupyter_from_json(root: &Value) -> Result<AST> {
    match root {
        Value::Object(o) => Ok(jupyter_root(o)),
        _ => Err(ParseError(String::from("Not a valid jupyter json"))),
    }
}

fn jupyter_root(dict: &Map<String, Value>) -> AST {
    if let Some(cells) = dict.get("cells") {
        if let Value::Array(v) = cells {
            return jupyter_cells(v);
        }
    }
    AST::statements(vec![])
}

fn jupyter_cells(cells: &Vec<Value>) -> AST {
    let mut out = vec![];
    for cell in cells {
        if let Value::Object(o) = cell {
            out.extend(jupyter_cell(o))
        }
    }
    AST::statements(out)
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
        match o.kind {
            ASTKind::Statements(v) => {
                return v;
            }
            _ => {}
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
