mod cli;
pub mod error;
pub mod storage;
pub mod task;

use anyhow::Result;
use storage::{load, save};
use task::TaskManager;

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
            let task_list = manager.list();

            // TODO: Generate a pretty formatted list to display.
            if task_list.is_empty() {
                println!("No tasks in the list.");
            } else {
                println!("{:?}", task_list);
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

    save(manager.list())?;

    Ok(())
}
