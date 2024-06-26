use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut command;
    loop {
        print!("sqlite> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut buffer)?;
        command = buffer.trim();

        match command {
            ".exit" => {
                return Ok(());
            }
            _ => {
                println!("Unrecognized command `{}`.", command);
                buffer.clear();
            }
        }
    }
}
