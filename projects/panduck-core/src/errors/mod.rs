mod basic;
#[cfg(feature = "docx-rs")]
mod docx;
#[cfg(feature = "html_parser")]
mod html;
#[cfg(feature = "serde_json")]
mod json;

use self::PanduckErrorKind::*;
use notedown_ast::Url;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
    path::Path,
};

pub type Result<T> = std::result::Result<T, PanduckError>;

#[derive(Debug)]
pub struct PanduckError {
    kind: PanduckErrorKind,
    file: Option<Url>,
    position: (usize, usize),
}

#[derive(Debug)]
pub enum PanduckErrorKind {
    IOError(std::io::Error),
    ParseError(String),
    UnsupportedFormat(String),
    Unknown(String),
}

// noinspection ALL
impl PanduckError {
    pub fn set_url(mut self, url: Url) -> Self {
        Self { kind: self.kind, file: Some(url), position: self.position }
    }
    pub fn set_path(mut self, path: impl AsRef<Path>) -> Self {
        match Url::from_directory_path(path) {
            Ok(url) => self.set_url(url),
            Err(_) => self,
        }
    }
    pub fn set_position(mut self, position: (usize, usize)) -> Self {
        Self { kind: self.kind, file: self.file, position }
    }
}

impl PanduckError {
    pub fn unsupported_file(msg: impl Into<String>) -> Self {
        Self { kind: UnsupportedFormat(msg.into()), file: None, position: (0, 0) }
    }
}

impl Default for PanduckError {
    fn default() -> Self {
        Self { kind: Unknown(String::new()), file: None, position: (0, 0) }
    }
}

impl Error for PanduckError {}

impl Display for PanduckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.file {
            None => {
                writeln!(f, "at [{}:{}] of <Anonymous>", self.position.0, self.position.1)?;
            }
            Some(s) => {
                writeln!(f, "at [{}:{}] of {}", self.position.0, self.position.1, s.as_str())?;
            }
        }
        Display::fmt(&self.kind, f)
    }
}

impl Display for PanduckErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
