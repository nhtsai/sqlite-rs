use std::error::Error;
use std::fmt;

// Meta Command Error
#[derive(Debug)]
pub struct MetaCommandError {
    details: String,
}

impl MetaCommandError {
    pub fn new(msg: &str) -> MetaCommandError {
        MetaCommandError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MetaCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized meta command '{}'.", self.details)
    }
}

impl Error for MetaCommandError {
    fn description(&self) -> &str {
        &self.details
    }
}

// Statement Error
#[derive(Debug)]
pub struct StatementError {
    details: String,
}

impl StatementError {
    pub fn new(msg: &str) -> StatementError {
        StatementError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for StatementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid statement '{}'.", self.details)
    }
}

impl Error for StatementError {
    fn description(&self) -> &str {
        &self.details
    }
}

// Execution Error
#[derive(Debug)]
pub struct ExecutionError;

// impl ExecutionError {
//     pub fn new() -> ExecutionError {
//         ExecutionError{}
//     }
// }

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: Table Full.")
    }
}

impl Error for ExecutionError {
    fn description(&self) -> &str {
        "Error: Table Full."
    }
}
