// use std::fmt;
use std::io::{self, prelude::*};
use std::process;

// mod error;
mod table;
use table::{Row, Table};

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

enum ExecuteResult {
    Success,
}

#[derive(Debug)]
enum Statement {
    Select,
    Insert(Row),
}

/// Prints the prompt for user input
fn print_prompt() {
    // immediately flush prompt to stdout, avoids line-buffering
    print!("rs-sqlite> ");
    io::stdout().flush().unwrap();
}

/// Reads input command from stdin
///
/// # Arguments
///
/// * `input` - A mutable String that will hold the input command
///
/// # Returns
///
/// A String of the parsed input command
fn read_input(mut input: String) -> String {
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    input.trim_end().to_string()
}

/// Performs meta commands
///
/// # Arguments
///
/// * `cmd` - A reference to a String that specifies the meta command
///
/// # Returns
///
/// A Result enum of either the String reference or a MetaCommandError
fn do_meta_command(cmd: &String) -> MetaCommandResult {
    match cmd.as_str() {
        ".exit" => {
            process::exit(0);
        }
        _ => MetaCommandResult::UnrecognizedCommand,
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
fn prepare_statement(cmd: &String) -> Result<Statement, PrepareResult> {
    if cmd.starts_with("select") {
        return Ok(Statement::Select);
    }

    if cmd.starts_with("insert") {
        // extract statement input
        let split: Vec<&str> = cmd.split_whitespace().collect();

        if split.len() != 4 {
            return Err(PrepareResult::UnrecognizedStatement);
        }

        // parse row id
        let row_id: u32 = match split[1].parse() {
            Ok(row_id) => row_id,
            Err(_) => {
                return Err(PrepareResult::UnrecognizedStatement);
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

        return Ok(Statement::Insert(row));
    }
    return Err(PrepareResult::UnrecognizedStatement);
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
fn execute_statement(statement: &Statement, table: &mut Table) -> ExecuteResult {
    match statement {
        Statement::Select => {
            println!("This is where we would do a select.");
            ExecuteResult::Success
        }
        Statement::Insert(_) => {
            println!("This is where we would do an insert.");
            ExecuteResult::Success
        }
    }
}

/// Main REPL that processes commands and statements from user input
fn main() {
    let mut input_buffer: String = String::new(); // consider String::with_capacity()
    let mut table: Table = Table {
        num_rows: 0,
        pages: [&mut 0; 100],
    };

    loop {
        print_prompt();

        // Read input as command
        input_buffer = read_input(input_buffer);

        // Handle meta commands
        if input_buffer.starts_with(".") {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::Success => {
                    println!("{} processed.", input_buffer);
                }
                MetaCommandResult::UnrecognizedCommand => {
                    println!("Unrecognized meta command: {}", input_buffer);
                }
            }
            continue;
        }

        // Handle statements
        let statement = match prepare_statement(&input_buffer) {
            Ok(s) => s,
            Err(_) => {
                println!("Invalid statement: {}", input_buffer);
                continue;
            }
        };
        println!("{:?}", statement);

        match execute_statement(&statement, &mut table) {
            ExecuteResult::Success => {
                println!("Executed.");
            }
        };
    }
}
