mod cli;
pub mod error;
pub mod storage;
pub mod task;

use error::TaskError;
use task::TaskManager;

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<(), TaskError> {
    let args = cli::parse();
    let mut manager = TaskManager::new();

    match args.command {
        cli::Command::Add { description } => {
            let task = manager.add(description)?;
            println!("Task added:\t {:?}", task);
        }
        cli::Command::List { completed, pending } => {
            let tasks = manager.list();

            if tasks.is_empty() {
                println!("No tasks in the list.");
            } else {
                println!("{:?}", tasks);
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
    // TODO: Save tasks back to storage

    Ok(())
}
