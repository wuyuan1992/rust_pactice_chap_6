// Write a Rust program that defines an enum called Task with two variants:
// Incomplete and Complete.
// The Incomplete variant should have an associated String field for the task name.
// The Complete variant should have two associated fields: a String
// for the task name and a u32 for the number of days since the task was completed.

// Implement a function called print_task that takes a Task object as an argument and
// prints a message to the console describing the task.
// If the task is incomplete, the message should say "Task: <name>".
// If the task is complete, the message should say "Task: <name> (completed <days> days ago)".

// Finally, write a main function that creates a vector of Task objects and uses a loop and
// pattern matching to print each task to the console using the print_task function.
// The loop should break when the user enters "q" at the prompt.

use std::io;

enum Task {
    Complete(String, u32),
    Incomplete(String),
}

impl Task {
    fn print(&self) {
        match self {
            Task::Complete(name, day) => {
                println!("Task: {} (completed {} days ago)", name, day)
            }
            Task::Incomplete(name) => {
                println!("Task: {}", name)
            }
        };
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(Task::Complete(String::from("Complete task 1"), 4));
    tasks.push(Task::Complete(String::from("Complete task 2"), 8));
    tasks.push(Task::Incomplete(String::from("InComplete task 1")));

    loop {
        let mut input = String::new();
        println!("Enter a command: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();

        match command {
            "show" => {
                for task in &tasks {
                    task.print();
                }
            }
            "q" => {
                break;
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}
