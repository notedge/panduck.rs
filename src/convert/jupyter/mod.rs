use anyhow::{Error, Result};
use notedown_ast::AST;
use serde_json::{Map, Value};

pub fn jupyter_from_json(root: &Value) -> Result<AST> {
    match root {
        Value::Object(o) => Ok(jupyter_root(o)),
        _ => Err(Error::msg("Not a vaild jupytre json")),
    }
}

fn jupyter_root(dict: &Map<String, Value>) -> AST {
    match dict.get("cells") {
        None => (),
        Some(cells) => match cells {
            Value::Object(o) => jupyter_cells(o),
            _ => (),
        },
    }

    println!("{:?}", dict);
    unimplemented!()
}

fn jupyter_cells(dict: &Map<String, Value>) -> AST {
    println!("{:?}", dict);
    unimplemented!()
}
