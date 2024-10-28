use crate::task::Task;
use std::fs::{File};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;
use std::io::BufRead;


const FILE_PATH: &str = "tasks.txt";

pub fn load_tasks() -> io::Result<Vec<Task>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut tasks = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let description = line?;
        tasks.push(Task::new(index as u32 + 1, description));
    }

    Ok(tasks)
}

pub fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let file = File::create(FILE_PATH)?;
    let mut writer = BufWriter::new(file);

    for task in tasks {
        writeln!(writer, "{}", task.description)?;
    }

    Ok(())
}
