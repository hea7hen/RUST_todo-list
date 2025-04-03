use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}
fn main() {
    let mut tasks: Vec<Task> = load_tasks().unwrap_or_else(|_| Vec::new());
    loop {
        println!("\nTo-Do List:");
        for (i, task) in tasks.iter().enumerate() {
            println!(
                "{} [{}]: {}",
                i + 1,
                if task.completed { "x" } else { " " },
                task.description
            );
    }
        println!("\nOptions:");
        println!("1. Add task");
        println!("2. Mark task as completed");
        println!("3. Remove task");
        println!("4. Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim().parse::<u32>() {
            Ok(1) => add_task(&mut tasks),
            Ok(2) => mark_completed(&mut tasks),
            Ok(3) => remove_task(&mut tasks),
            Ok(4) => break,
            _ => println!("Invalid choice."),
        }

        save_tasks(&tasks).expect("Failed to save tasks.");
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    print!("Enter task description: ");
    io::stdout().flush().unwrap();

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");

    tasks.push(Task {
        description: description.trim().to_string(),
        completed: false,
    });
}

fn mark_completed(tasks: &mut Vec<Task>) {
    print!("Enter task number to mark as completed: ");
    io::stdout().flush().unwrap();

    let mut task_num = String::new();
    io::stdin().read_line(&mut task_num).expect("Failed to read line");

    if let Ok(num) = task_num.trim().parse::<usize>() {
        if num > 0 && num <= tasks.len() {
            tasks[num - 1].completed = true;
        } else {
            println!("Invalid task number.");
        }
    } else {
        println!("Invalid input.");
    }
}
fn remove_task(tasks: &mut Vec<Task>) {
    print!("Enter task number to remove: ");
    io::stdout().flush().unwrap();

    let mut task_num = String::new();
    io::stdin().read_line(&mut task_num).expect("Failed to read line");

    if let Ok(num) = task_num.trim().parse::<usize>() {
        if num > 0 && num <= tasks.len() {
            tasks.remove(num - 1);
        } else {
            println!("Invalid task number.");
        }
    } else {
        println!("Invalid input.");
    }
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_string(tasks)?;
    fs::write("tasks.json", data)?;
    Ok(())
}
fn load_tasks() -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let path = Path::new("tasks.json");

    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(path)?;
    let tasks = serde_json::from_str(&data)?;
    Ok(tasks)
}