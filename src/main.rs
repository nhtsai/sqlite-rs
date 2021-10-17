use std::io::{self, prelude::*};

fn main() {
    let mut cmd: String = String::new(); // consider String::with_capacity()
    loop {
        print_prompt();
        
        // Read in commandSt
        io::stdin().read_line(&mut cmd).expect("Failed to read command.");
        
        
        cmd = cmd.trim().to_string();
        
        match cmd.as_str() {
            ".exit" => {break;}
            _ => {
                println!("Unrecognized command '{}'.", cmd);
                cmd = String::new();
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}