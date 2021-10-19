use std::fmt;
use std::io::{self, prelude::*};
use std::process;

mod error;

const MAX_TABLE_ROWS: u32 = 100;

#[derive(Debug)]
enum Statement {
    Select,
    Insert(Row),
}

#[derive(Debug)]
struct Row {
    id: u32,
    username: String,
    email: String,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}

struct Table {
    num_rows: u32,
    pages: [*mut u8; 100],
}

fn main() {
    let mut cmd: String = String::new(); // consider String::with_capacity()
    let mut table: Table = Table {
        num_rows: 0,
        pages: [&mut 0; 100],
    };
    loop {
        print_prompt();

        // Read in command
        cmd = read_input(cmd);

        // Handle meta commands
        let first_char: char = cmd.chars().next().unwrap();
        if first_char == '.' {
            match do_meta_command(&cmd) {
                Ok(cmd) => {
                    println!("{} processed.", cmd);
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
            continue;
        }

        // Handle statements
        let statement = match get_statement(&cmd) {
            Ok(s) => s,
            Err(error) => {
                println!("{}", error);
                continue;
            }
        };
        println!("{:?}", statement);

        match execute_statement(&statement, &mut table) {
            Ok(_) => {
                println!("Executed.");
            }
            Err(error) => {
                println!("{}", error);
            }
        };
    }
}

fn print_prompt() {
    // immediately flush prompt to stdout, avoids line-buffering
    print!("rs-sqlite> ");
    io::stdout().flush().unwrap();
}

fn read_input(mut cmd: String) -> String {
    cmd.clear();
    io::stdin()
        .read_line(&mut cmd)
        .expect("Failed to read command.");
    cmd.trim().to_string()
}

fn do_meta_command(cmd: &String) -> Result<&String, error::MetaCommandError> {
    match cmd.as_str() {
        ".exit" => process::exit(0),
        _ => Err(error::MetaCommandError::new(cmd)),
    }
}

/// Creates and returns a Statement from a specified command
///
/// # Arguments
///
/// * `cmd` - A String that specifies the command from the prompt
///
/// # Returns
///
/// A Result enum containing either a Statement or StatementError
fn get_statement(cmd: &String) -> Result<Statement, error::StatementError> {
    if cmd.len() < 6 {
        return Err(error::StatementError::new(cmd));
    }

    match &cmd.to_lowercase()[0..6] {
        "insert" => {
            // extract statement input
            let split: Vec<&str> = cmd.split_whitespace().collect();

            if split.len() != 4 {
                return Err(error::StatementError::new(cmd));
            }

            // parse row id
            let row_id: u32 = match split[1].parse() {
                Ok(row_id) => row_id,
                Err(_) => {
                    return Err(error::StatementError::new(cmd));
                }
            };

            // parse row username
            let row_username: String = String::from(split[2]);
            let row_email: String = String::from(split[3]);
            let row: Row = Row {
                id: row_id,
                username: row_username,
                email: row_email,
            };

            Ok(Statement::Insert(row))
        }
        "select" => Ok(Statement::Select),
        _ => Err(error::StatementError::new(cmd)),
    }
}

/// Executes a statement on the table
///
/// # Arguments
///
/// * `statement` - A Statement enum variant that specifies which operation to perform
/// * `table` - A Table struct that specifies will be queried or manipulated
///
/// # Returns
///
/// A Result enum containing either () or an ExecutionError
fn execute_statement(
    statement: &Statement,
    table: &mut Table,
) -> Result<(), error::ExecutionError> {
    if table.num_rows > MAX_TABLE_ROWS {
        return Err(error::ExecutionError);
    }

    match statement {
        Statement::Insert(_) => {
            println!("This is where we would do an insert.");
            Ok(())
        }
        Statement::Select => {
            println!("This is where we would do a select.");
            Ok(())
        }
    }
}
