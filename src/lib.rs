use std::{fs};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use rand::{Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
}

impl Task {
    pub fn new (args: Vec<String>) -> Result<Task, &'static str> {
        let args = args.clone();
        let mut args_iter = args.iter();
        let random_number: u32 = rand::thread_rng().gen_range(1..=100000000);

        args_iter.next();
        args_iter.next();

        let title= match args_iter.next() {
            Some(arg) => Ok(arg),
            None => Err("Title was not provided"),
        }?.to_string();

        Ok(Task {
            id: random_number,
            title,
        })
    }
}

pub struct Tasks {
    pub tasks: Vec<Task>,
    pub file: File
}

impl Tasks {
    pub fn new(tasks: Vec<Task>, file: File) -> Tasks {
        Tasks {
            tasks,
            file,
        }
    }

    pub fn add_new_task (
        &mut self, task: Task) {
        self.tasks.push(task);
    }
    pub fn remove_task(&mut self, id: u32) -> bool {
        let initial_len = self.tasks.len();

        self.tasks.retain(|t| t.id != id);
        // Return true if the length changed (i.e., a task was removed)
        self.tasks.len() != initial_len
    }
    pub fn save_tasks(&mut self) -> Result<(), &'static str> {
        // Serialize tasks to a temporary string
        let json_string = match serde_json::to_string_pretty(&self.tasks) {
            Ok(s) => s,
            Err(_) => return Err("Failed to serialize tasks"),
        };

        // Truncate the file to size zero, effectively clearing its content
        if let Err(_) = self.file.seek(SeekFrom::Start(0)) {
            return Err("Failed to seek to the beginning of the file");
        }

        if let Err(_) = self.file.set_len(0) {
            return Err("Failed to truncate the file");
        }

        // Write the serialized string to the file
        if let Err(_) = self.file.write_all(json_string.as_bytes()) {
            return Err("Failed to save tasks to file");
        }

        // Flush the file to ensure changes are immediately written to disk
        if let Err(_) = self.file.flush() {
            return Err("Failed to flush the file");
        }

        Ok(())
    }
}

pub fn mark_as_done(args: Vec<String>, tasks: &mut Tasks) -> Result<(), &'static str> {
    let id: u32 = match args[2].parse() {
        Ok(parsed_id) => parsed_id,
        Err(_) => return Err("Invalid ID provided"),
    };

    if tasks.remove_task(id) {
        if let Err(err) = tasks.save_tasks() {
            eprintln!("Error saving tasks: {}", err);
            return Err("Failed to save tasks");
        }
        println!("The task was removed successfully");
        Ok(())
    } else {
        Err("Task with the specified ID not found")
    }
}

pub fn read_all_tasks (file_path: &str) -> Vec<Task> {
    if let Ok(contents) = fs::read_to_string(file_path) {
        if let Ok(tasks) = serde_json::from_str::<Vec<Task>>(&contents) {
            return tasks
        }
    }

    Vec::new()
}

pub fn list_all_todos(file_path: &str) {
    let tasks = read_all_tasks(file_path);
    println!("---------------------------------");
    println!("ID        | Task Title");
    println!("---------------------------------");
    for t in tasks {
        println!("{} | {}", t.id, t.title);
        println!("---------------------------------");
    }
}

pub fn create_new(args: Vec<String>, tasks: &mut Tasks) {
    let task = Task::new(args).unwrap();

    tasks.add_new_task(task);
    tasks.save_tasks().expect("Error occurred during saving the file");
}