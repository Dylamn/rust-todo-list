mod cli;
pub mod error;
pub mod storage;
pub mod task;

use anyhow::Result;
use storage::{load, save};
use task::{Task, TaskManager};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<()> {
    let args = cli::parse();
    let tasks = load()?;
    let mut manager = TaskManager::new(tasks);

    match args.command {
        cli::Command::Add { description } => {
            let task = manager.add(description)?;
            println!("Task added:\t {:?}", task);
        }
        cli::Command::List { completed, pending } => {
            let task_list = manager.list(completed, pending).collect::<Vec<&Task>>();

            // TODO: Generate a pretty formatted list to display.
            if task_list.is_empty() {
                println!("No tasks in the list.");
            } else {
                for task in task_list {
                    let is_done: char = if task.completed_at.is_some() {
                        'X'
                    } else {
                        ' '
                    };

                    println!("{}. {} ({})", task.id, task.description, is_done);
                }
            }
        }
        cli::Command::Done { id } => {
            manager.mark_done(id)?;
            println!("Task with ID {} marked as done.", id);
        }
        cli::Command::Remove { id } => {
            manager.remove(id)?;
            println!("Task with ID {} has been removed.", id);
        }
    }

    let tasks: Vec<Task> = manager.list(true, true).cloned().collect();
    save(&tasks)?;

    Ok(())
}
