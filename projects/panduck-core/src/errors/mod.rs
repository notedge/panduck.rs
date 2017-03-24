mod basic;
#[cfg(feature = "docx-rs")]
mod docx;
#[cfg(feature = "html_parser")]
mod html;
#[cfg(feature = "serde_json")]
mod json;

use self::PanduckErrorKind::*;
use notedown_parser::utils::Url;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub type Result<T> = std::result::Result<T, PanduckError>;

#[derive(Clone, Debug)]
pub struct PanduckError {
    kind: PanduckErrorKind,
    file: Option<Url>,
    position: (usize, usize),
}

#[derive(Clone, Debug)]
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
    pub fn set_position(mut self, position: (usize, usize)) {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let file = self.file.and_then(|e| e.to_file_path().ok()).and_then(|e| e.to_str());
        let file = match file {
            None => "<Anonymous>",
            Some(s) => s,
        };
        writeln!(f, "at [{}:{}] of {}", self.position.0, self.position.1, file)?;
        Display::fmt(&self.kind, f)
    }
}

impl Display for PanduckErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
