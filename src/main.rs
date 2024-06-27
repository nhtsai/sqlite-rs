use std::error::Error;
use std::fmt;
use std::io::{self, Write};

#[derive(fmt::Debug)]
struct MetaCommandError {
    details: String,
}

impl MetaCommandError {
    fn new(msg: &str) -> MetaCommandError {
        MetaCommandError {
            details: msg.to_string(),
        }
    }
}

impl Error for MetaCommandError {}

impl fmt::Display for MetaCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized command `{}`.", self.details)
    }
}

#[derive(Debug)]
struct InvalidStatementError {
    details: String,
}

impl InvalidStatementError {
    fn new(msg: &str) -> InvalidStatementError {
        InvalidStatementError {
            details: msg.to_string(),
        }
    }
}

impl Error for InvalidStatementError {}

impl fmt::Display for InvalidStatementError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized keyword at start of `{}`.", self.details)
    }
}

fn main() {
    let mut buffer = String::new();
    let mut command: &str;
    loop {
        print!("sqlite-rs> ");
        io::stdout().flush().unwrap();

        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        command = buffer.trim();

        // process meta command
        match command {
            meta_command if command.starts_with('.') => {
                if meta_command == ".exit" {
                    break;
                }
                if let Err(e) = do_meta_command(meta_command) {
                    eprintln!("{e}");
                };
            }
            command => match prepare_statement(command) {
                Ok(statement) => {
                    execute_statement(&statement);
                }
                Err(e) => {
                    eprintln!("{e}");
                }
            },
        }
    }
}

fn do_meta_command(command: &str) -> Result<(), MetaCommandError> {
    match command {
        _ => Err(MetaCommandError::new(command)),
    }
}

#[derive(Debug)]
enum Statement {
    Select,
    Insert,
}

fn prepare_statement(command: &str) -> Result<Statement, InvalidStatementError> {
    match command {
        command if command.to_lowercase().starts_with("insert") => Ok(Statement::Insert),
        command if command.to_lowercase().starts_with("select") => Ok(Statement::Select),
        _ => Err(InvalidStatementError::new(command)),
    }
}

fn execute_statement(statement: &Statement) {
    match statement {
        Statement::Insert => {
            println!("Executing insert statement.");
        }
        Statement::Select => {
            println!("Executing select statement.");
        }
    }
}
