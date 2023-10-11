use std::{error, fmt, num};

/// Error that is returned if lexer fails
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum Error {
    /// Lexer failed to process all input
    #[default]
    LexingIncomplete,
    /// Lexer failed for unknow reasons
    ParseIntError(num::ParseIntError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

impl From<num::ParseIntError> for Error {
    fn from(o: num::ParseIntError) -> Error {
        Error::ParseIntError(o)
    }
}
