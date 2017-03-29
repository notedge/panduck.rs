use crate::{PanduckError, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::read_to_string,
    path::Path,
    rc::Rc,
};
use notedown_ast::ASTNode;

pub struct ExtensionRegistrar {
    arena: Vec<Rc<ExtensionHandler>>,
    inner: BTreeMap<String, BTreeSet<Rc<ExtensionHandler>>>,
}

pub struct ExtensionHandler {
    try_extension: BTreeSet<String>,
    parser: fn(&str) -> Result<ASTNode>,
}

impl ExtensionRegistrar {
    pub fn insert(&mut self, h: ExtensionHandler) {
        let handler = Rc::new(h);
        self.arena.push(Rc::clone(&handler));
        for i in handler.try_extension {
            let new = Rc::clone(&handler);
            let ptr = self.inner.get_mut(&i);
            match ptr {
                Some(s) => s.insert(new),
                None => {
                    let mut s = BTreeSet::new();
                    s.insert(new);
                    self.inner.insert(i, s)
                }
            }
        }
    }
    pub fn get(&self, ext: &str) -> &BTreeSet<Rc<ExtensionHandler>> {
        match self.inner.get(ext) {
            None => &BTreeSet::new(),
            Some(s) => s,
        }
    }
    pub fn parse_by_ext(&self, file: impl AsRef<Path>) -> Result<ASTNode> {
        let ext = file.as_ref().extension().unwrap_or_default().to_str().unwrap_or_default();
        let input = &read_to_string(file.as_ref())?;
        for t in self.get(ext) {
            let parse = t.parser;
            match parse(input) {
                Ok(s) => return Ok(s),
                Err(_) => continue,
            }
        }
        let mut error = PanduckError::unsupported_file(ext);
        error.set_path();

        return Err(error);
    }
}
