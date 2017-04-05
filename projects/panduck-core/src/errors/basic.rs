use super::*;
use std::string::FromUtf8Error;

impl From<std::io::Error> for PanduckError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: PanduckErrorKind::IOError(e), file: None, position: (0, 0) }
    }
}

impl From<()> for PanduckError {
    fn from(_: ()) -> Self {
        Self { kind: PanduckErrorKind::Unknown, file: None, position: (0, 0) }
    }
}

impl From<FromUtf8Error> for PanduckError {
    fn from(e: FromUtf8Error) -> Self {
        Self { kind: PanduckErrorKind::ParseError(e.to_string()), file: None, position: (0, 0) }
    }
}



// impl From<notedown_parser::Error> for PanduckError {
//     fn from(e: notedown_parser::Error) -> Self {
//         match e {
//             notedown_parser::Error::LexerError(s) => Self::ParseError(s),
//             notedown_parser::Error::FileNotFound(s) => Self::IOError(s),
//             notedown_parser::Error::IOError(s) => Self::IOError(s),
//         }
//     }
// }
