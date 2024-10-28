use crate::task::Task;
use crate::file_handler;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Add(String),
    Remove(u32),
    List,
    Save,
    Quit,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match parts[0] {
            "add" => Ok(Command::Add(parts[1..].join(" "))),
            "remove" => {
                if parts.len() < 2 {
                    return Err("Specify a task ID to remove".into());
                }
                let id = parts[1].parse().map_err(|_| "Invalid ID".to_string())?;
                Ok(Command::Remove(id))
            },
            "list" => Ok(Command::List),
            "save" => Ok(Command::Save),
            "quit" => Ok(Command::Quit),
            _ => Err("Unknown command".into()),
        }
    }
}

pub fn execute_command(cmd: Command, tasks: &mut Vec<Task>) -> bool {
    match cmd {
        Command::Add(description) => {
            let id = tasks.len() as u32 + 1;
            let task = Task::new(id, description);
            tasks.push(task);
            println!("Task added!");
        },
        Command::Remove(id) => {
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                tasks.remove(pos);
                println!("Task removed!");
            } else {
                println!("Task ID not found!");
            }
        },
        Command::List => {
            for task in tasks {
                println!("{}: {}", task.id, task.description);
            }
        },
        Command::Save => {
            if let Err(err) = file_handler::save_tasks(tasks) {
                println!("Error saving tasks: {}", err);
            } else {
                println!("Tasks saved successfully!");
            }
        },
        Command::Quit => {
            println!("Exiting...");
            return false;
        },
    }
    true
}
