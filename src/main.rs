extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::io::Write;

const FILE_PATH: &str = "./tasks.json";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    title: String,
    description: String,
    is_done: bool,
}

impl Task {
    fn new(title: String, description: String) -> Self {
        Task {
            title,
            description,
            is_done: false,
        }
    }
}

fn read_tasks_from_file() -> Result<Vec<Task>, serde_json::Error> {
    let file = fs::read_to_string(FILE_PATH)
        .unwrap_or("[]".into());
    serde_json::from_str(file.as_str())
}

fn write_tasks_to_file(tasks: &Vec<Task>) -> Result<(), io::Error> {
    let tasks = serde_json::to_string_pretty(tasks).unwrap();
    fs::File::create(FILE_PATH)?
        .write_all(tasks.as_bytes())
}

fn change_task_state(task: &mut Task, state: bool) {
    task.is_done = state;
}

fn print_tasks(tasks: &Vec<Task>) {
    for (i, task) in tasks.iter().enumerate() {
        let is_done = if task.is_done { "x" } else { " " };
        println!("{}) [{}] {}: {}", i, is_done, task.title, task.description);
    }
}

fn main() {
    let mut tasks = read_tasks_from_file().unwrap();
    let command_list = [
        "Add new task",
        "Remove task",
        "Change task state",
        "Show tasks",
        "Exit"
    ];

    loop {
        for (i, &commands) in command_list.iter().enumerate() {
            println!("{}: {}", i + 1, commands);
        }

        print!("*: ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command: u8 = match command.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match command {
            1 => {
                print!("title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();
                let title = title.trim().to_owned();

                print!("description: ");
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                let description = description.trim().to_owned();

                let new_task = Task::new(title, description);
                tasks.push(new_task);
                write_tasks_to_file(&tasks).unwrap();
            },
            2 => {
                print_tasks(&tasks);
                print!("index: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let index = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                tasks.remove(index);
                write_tasks_to_file(&tasks).unwrap();
            },
            3 => {
                print_tasks(&tasks);
                print!("index: ");
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let index: usize = match index.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                let task: &mut Task = match tasks.get_mut(index) {
                    Some(task) => task,
                    None => continue,
                };
                change_task_state(task, !task.is_done);
                write_tasks_to_file(&tasks).unwrap();
            }
            4 => {
                print_tasks(&tasks);
            }
            5 => {
                break;
            }
            _ => (),
        };
    }
}
