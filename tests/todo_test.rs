// tests/tasks_test.rs
use std::{env, fs, io};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Seek, SeekFrom};
use std::string::String;
use todo_list::{mark_as_done, read_all_tasks, Task, Tasks};

const TEST_FILE_PATH: &str = "test_todo.json";

fn setup() -> Tasks {
    let fs = match OpenOptions::new().read(true).write(true).truncate(true).open(&TEST_FILE_PATH) {
        Ok(fs) => fs,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match fs::File::create(&TEST_FILE_PATH) {
                Ok(fs) => fs,
                Err(e) => {
                    eprintln!("Error creating the todo file: {}", e);
                    // Return a default file to ensure both arms have the same type
                    File::open(&TEST_FILE_PATH).expect("Failed to open the file")
                }
            }
        }
        Err(e) => {
            eprintln!("Error while opening the file: {}", e);
            // Return a default file to ensure both arms have the same type
            File::open(&TEST_FILE_PATH).expect("Failed to open the file")
        }
    };

    let tasks: Tasks = Tasks::new(read_all_tasks(&TEST_FILE_PATH), fs);

    tasks
}

fn cleanup() -> io::Result<()> {
    let mut tasks = setup();

    // Set the length of the file to 0, effectively clearing its contents
    tasks.file.set_len(0)?;

    Ok(())
}

#[test]
fn test_add_new_task() {
    let mut tasks = setup();

    let task = Task {
        id: 123,
        title: String::from("Test Task")
    };

    tasks.add_new_task(task);

    assert_eq!(tasks.tasks.len(), 1);
    assert_eq!(tasks.tasks[0].title, "Test Task");

    cleanup().expect("Cleanup Error");
}

#[test]
fn test_remove_task() {
    let mut tasks = setup();

    let task = Task {
        id: 123,
        title: "Test Task".to_string(),
    };

    tasks.add_new_task(task.clone());

    assert_eq!(tasks.tasks.len(), 1);

    let removed = tasks.remove_task(task.id);

    assert!(removed);
    assert_eq!(tasks.tasks.len(), 0);

    cleanup().expect("Cleanup Error");
}

#[test]
fn test_mark_as_done() {
    let mut tasks = setup();

    let task = Task {
        id: 100,
        title: String::from("Test Task 1")
    };

    tasks.add_new_task(task);
    tasks.save_tasks().expect("Error during saving the tasks");

    assert_eq!(tasks.tasks.len(), 1);

    let args = vec!["todo".to_string(), "--done".to_string(), "100".to_string()];
    mark_as_done(args, &mut tasks).expect("Failed to mark task as done");

    eprintln!("Length: {}", tasks.tasks.len());
    assert_eq!(tasks.tasks.len(), 0);

    cleanup().expect("Cleanup Error");
}

#[test]
fn test_read_all_tasks() {
    let mut tasks = setup();

    let task = Task {
      id: 123, title: String::from("Test Title 1")
    };

    let task1 = Task {
        id: 124,
        title: String::from("Test Title 2")
    };

    tasks.add_new_task(task);
    tasks.add_new_task(task1);

    tasks.save_tasks().expect("Error during saving the tasks");

    assert_eq!(tasks.tasks.len(), 2);

    cleanup().expect("Cleanup Error");
}