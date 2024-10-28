mod task;
mod commands;
mod file_handler;

use commands::{Command, execute_command};
use std::io::{self, Write};

fn main() {
    let mut tasks = file_handler::load_tasks().unwrap_or_else(|err| {
        println!("Error loading tasks: {}", err);
        vec![]
    });

    loop {
        println!("\nEnter a command: (add, remove, list, save, quit)");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let command = input.trim().parse::<Command>();

        match command {
            Ok(cmd) => {
                if !execute_command(cmd, &mut tasks) {
                    break;
                }
            },
            Err(err) => println!("Error: {}", err),
        }
    }
}
