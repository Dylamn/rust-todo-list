use std::collections::HashMap;
use std::io;

// 1. Add a task:
//    Allow the user to add a task which has :
//     - title
//     - description
// 2. Display the tasks list:
//    Show if they're done or not.
// 3. Mark a task as completed:
//    Allow the user to mark a task as completed
// 4. Delete a task:
//    Allow the user to delete a specific task from the list
// 5. Leave the application:
//    Allow the user to leave the application
fn main() {
    println!("Hello and welcome to To-do List!");

    let mut tasks: HashMap<i32, (String, TaskStatus)> = HashMap::new();
    let mut next_task_id = 1;

    loop {
        println!("1. Add a task");
        println!("2. Display the list");
        println!("3. Mark a task as done");
        println!("4. Remove a task");
        println!("5. Quit");

        let choice = ask_user();

        match choice.as_str() {
            "1" => {
                println!("Please enter a description for the task:");
                let description = ask_user();
                add_task(&mut tasks, description, &mut next_task_id);
            }
            "2" => {
                list_tasks(&tasks);
            }
            "3" => {
                println!("Please enter the task ID to mark as done:");
                let id_str = ask_user();
                if let Ok(task_id) = id_str.trim().parse::<i32>() {
                    mark_task_as_done(&mut tasks, task_id);
                } else {
                    println!("Invalid task ID.");
                }
            }
            "4" => {
                println!("Please enter the task ID to remove:");
                let id_str = ask_user();
                if let Ok(task_id) = id_str.trim().parse::<i32>() {
                    remove_task(&mut tasks, task_id);
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Not a valid choice. Please try again."),
        }
    }
}

#[derive(Debug)]
enum TaskStatus {
    Complete,
    Incomplete,
}

trait Task {
    fn description(&self) -> String;
    fn status(&self) -> TaskStatus;
}

impl Task for String {
    fn description(&self) -> String {
        self.clone()
    }

    fn status(&self) -> TaskStatus {
        TaskStatus::Incomplete
    }
}

fn add_task<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, description: T, task_id: &mut i32) {
    tasks.insert(*task_id, (description, TaskStatus::Incomplete));
    println!("Task n°{} added.", *task_id);
    println!("-----");
    *task_id += 1;
}

fn list_tasks<T: Task>(tasks: &HashMap<i32, (T, TaskStatus)>) -> () {
    if tasks.is_empty() {
        println!("There are no tasks to display.");
        return;
    }

    println!("List of tasks:");
    for (id, (description, status)) in tasks {
        println!("{}. {} ({:?})", id, description.description(), status);
    }
    println!("-----")
}

fn mark_task_as_done<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, task_id: i32) {
    if let Some((_, status)) = tasks.get_mut(&task_id) {
        *status = TaskStatus::Complete;
        println!("Task n° {} is done!", task_id);
    } else {
        println!("Invalid task id. Skipped...");
    }
}

fn remove_task<T: Task>(tasks: &mut HashMap<i32, (T, TaskStatus)>, task_id: i32) {
    if tasks.remove(&task_id).is_some() {
        println!("Task n°{} has been removed!", task_id);
    } else {
        println!("Invalid task id. Skipping...");
    }
}

fn ask_user() -> String {
    loop {
        let mut input = String::new();

        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("There was an error reading input. Please try again.");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            println!("Looks like input is empty. Please try again.");
            continue;
        }

        return trimmed.to_string();
    }
}
