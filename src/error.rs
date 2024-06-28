use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SQLiteError {
    InvalidCommand(String),
    InvalidStatement(String),
}

impl fmt::Display for SQLiteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SQLiteError::InvalidCommand(e) => write!(f, "Unrecognized command: `{e}`"),
            SQLiteError::InvalidStatement(e) => write!(f, "Unrecognized keyword at start of `{e}`"),
        }
    }
}

impl Error for SQLiteError {}
