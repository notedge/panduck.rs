#[derive(Clone, Debug)]
pub enum Error {
    IOError(String),
    ParseError(String),
    UnsupportedFormat(String),
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(format!("{}", e))
    }
}

impl From<notedown_parser::Error> for Error {
    fn from(e: notedown_parser::Error) -> Self {
        match e {
            notedown_parser::Error::LexerError(s) => Self::ParseError(s),
            notedown_parser::Error::FileNotFound(s) => Self::IOError(s),
            notedown_parser::Error::IOError(s) => Self::IOError(s),
        }
    }
}

impl From<html_parser::Error> for Error {
    fn from(e: html_parser::Error) -> Self {
        match e {
            html_parser::Error::Parsing(s) => Self::ParseError(format!("{}", s)),
            html_parser::Error::IO(s) => Self::IOError(format!("{}", s)),
            _ => Self::Unknown(format!("{}", e)),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::ParseError(format!("{}", e))
    }
}
