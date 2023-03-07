use chrono::{serde::ts_seconds, DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Utc> = Utc::now();
        Task { text, created_at }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let created_at = self.created_at.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]", self.text, created_at)
    }
}

fn retrieve_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.

    let tasks: Vec<Task> = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}

pub fn add_task(task: Task, path: PathBuf) -> Result<()> {
    //Open the file
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;

    //Open the contents of the file and add to a vector of tasks
    let mut tasks: Vec<Task> = retrieve_tasks(&file)?;
    // Write the modified task list back into the file.
    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn complete_task(path: PathBuf, task_position: usize) -> Result<()> {
    //Open the file
    let file = OpenOptions::new().read(true).write(true).open(path)?;
    //Open the contents of the file and add to a vector of tasks
    let mut tasks: Vec<Task> = retrieve_tasks(&file)?;
    //Try remove the task from the vector
    if task_position == 0 || task_position > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid tas Id"));
    }
    tasks.remove(task_position);
    //Write the modified task list back into the file.
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}
pub fn list_tasks(path: PathBuf) -> Result<()> {
    // Open the file
    let file = OpenOptions::new().read(true).open(path)?;
    // Parse the file and retrieve the tasks
    let tasks: Vec<Task> = retrieve_tasks(&file)?;
    // loop through the tasks and display them if any
    if tasks.is_empty() {
        println!("No tasks found");
    } else {
        let mut order = 1;
        for task in &tasks {
            println!("Order: {}Task {}", order, task);
            order += 1;
        }
    }
    Ok(())
}
