use serde_json::{Map, Value};
use wasi_notedown::exports::notedown::core::syntax_tree::NotedownRoot;
use wasi_notedown::{NotedownError, Result};
use wasi_notedown::exports::notedown::core::types::{Object, SyntaxError, TextRange};

pub fn parse_jupyter(text: &str) -> Result<NotedownRoot> {
    match serde_json::from_str::<Value>(text) {
        Ok(o) => {Ok(jupyter_from_json(o)?)}
        Err(e) => Err(NotedownError::Syntax(SyntaxError {
            reason: e.to_string(),
            file: None,
            range: TextRange { head_offset: 0, tail_offset: 0 },
        }))?,
    }
    
}

pub fn jupyter_from_json(root: Value) -> Result<NotedownRoot> {
    match root {
        Value::Object(o) => Ok(jupyter_root(o)),
        _ => Err(panic!("Not a valid jupyter json")),
    }
}

fn jupyter_root(dict: Map<String, Value>) -> NotedownRoot {
    if let Some(cells) = dict.get("cells") {
        if let Value::Array(v) = cells {
            return jupyter_cells(v);
        }
    }
    NotedownRoot {
        blocks: vec![],
        config: Object { map: vec![] },
        path: None,
    }
}

fn jupyter_cells(cells: &Vec<Value>) -> NotedownRoot {
    let mut out = vec![];
    for cell in cells {
        if let Value::Object(o) = cell {
            out.extend(jupyter_cell(o))
        }
    }
    NotedownRoot {
        blocks: vec![],
        config: Object { map: vec![] },
        path: None,
    }
}

fn jupyter_cell(dict: &Map<String, Value>) -> Vec<ASTNode> {
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

fn jupyter_markdown(dict: &Value) -> Vec<ASTNode> {
    let lines: Vec<String> = match dict {
        Value::Array(v) => v.into_iter().map(|v| jupyter_string(v)).collect(),
        _ => vec![],
    };
    match parse_common_markdown(&lines.join("\n")) {
        Ok(o) => {
            match o.value {
                ASTKind::Statements(v) => {
                    return v;
                }
                _ => {}
            }
        }
        _ => {}
    }
    vec![]
}

fn jupyter_string(o: &Value) -> String {
    match o {
        Value::String(v) => v.to_owned(),
        _ => String::new(),
    }
}
