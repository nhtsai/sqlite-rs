use std::io::{self, prelude::*};
use std::process;
mod error;

enum StatementType {
    Select,
    Insert,
}

struct Statement {
    stype: StatementType,
}

fn main() {
    let mut cmd: String = String::new(); // consider String::with_capacity()
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
                },
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

        execute_statement(&statement);
        println!("Executed.");
    }
}

fn print_prompt() {
    // immediately flush prompt to stdout, avoids line-buffering
    print!("rs-sqlite> ");
    io::stdout().flush().unwrap();
}

fn read_input(mut cmd: String) -> String {
    cmd.clear();
    io::stdin().read_line(&mut cmd).expect("Failed to read command.");
    cmd.trim().to_string()
}

fn do_meta_command(cmd: &String) -> Result<&String, error::MetaCommandError> {
    match cmd.as_str() {
        ".exit" => process::exit(0),
        _ => Err(error::MetaCommandError::new(cmd)),
    }
}

fn get_statement(cmd: &String) -> Result<Statement, error::StatementError> {
    if cmd.len() < 6 {
        return Err(error::StatementError::new(cmd));
    }

    match &cmd[0..6] {
        "insert" => Ok(Statement{stype: StatementType::Insert}),
        "select" => Ok(Statement{stype: StatementType::Select}),
        _ => Err(error::StatementError::new(cmd)),
    }
}

fn execute_statement(statement: &Statement) {
    match statement.stype {
        StatementType::Insert => {
            println!("This is where we would do an insert.");
        }
        StatementType::Select => {
            println!("This is where we would do a select.");
        }
    }
}