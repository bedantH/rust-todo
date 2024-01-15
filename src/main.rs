use std::env;
use std::fs;
use std::fs::OpenOptions;
use std::io::{ErrorKind};

use todo_list::{create_new, mark_as_done, list_all_todos, read_all_tasks, Tasks};

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    let file_path = String::from("todo.json");

    let fs = match OpenOptions::new().read(true).write(true).open(&file_path) {
        Ok(fs) => fs,
        Err(ref err) if err.kind() == ErrorKind::NotFound => {
            match fs::File::create(&file_path) {
                Ok(fs) => fs,
                Err(e) => {
                    eprintln!("Error creating the todo file: {}", e);
                    return;
                }
            }
        }
        Err(e) => {
            eprintln!("Error while opening the file: {}", e);
            return;
        }
    };

    let mut tasks: Tasks = Tasks::new(read_all_tasks(&file_path), fs);

    match args.len() {
        0 => eprintln!("No parameters were provided"),
        // cmd: todo
        1 => list_all_todos(&file_path),
        // cmd: todo --add title or todo --done id
        3 => if args.contains(&"--add".to_owned()) {
            create_new(args, &mut tasks)
        } else {
            mark_as_done(args, &mut tasks).expect("Error occurred during marking");
        },
        _ => println!("Default Case")
    }
}
