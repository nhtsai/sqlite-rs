use std::io::{self, Write};

mod error;

use error::SQLiteError;

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

fn do_meta_command(command: &str) -> Result<(), SQLiteError> {
    match command {
        _ => Err(SQLiteError::InvalidCommand(command.to_string())),
    }
}

#[derive(Debug)]
enum Statement {
    Select,
    Insert,
}

fn prepare_statement(command: &str) -> Result<Statement, SQLiteError> {
    match command {
        command if command.to_lowercase().starts_with("insert") => Ok(Statement::Insert),
        command if command.to_lowercase().starts_with("select") => Ok(Statement::Select),
        _ => Err(SQLiteError::InvalidStatement(command.to_string())),
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
