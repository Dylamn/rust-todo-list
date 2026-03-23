mod cli;
pub mod config;
pub mod error;
pub mod storage;
pub mod task;

use anyhow::Result;
use config::logging::resolve_log_level;
use log::LevelFilter;
use task::{Task, TaskManager};

fn main() {
    if let Err(e) = run() {
        println!("{}", e);
    }
}

fn run() -> Result<()> {
    let args = cli::parse();

    let log_level = resolve_log_level(args.verbose, args.log_level.clone());
    init_logger(log_level);

    let file_path = config::resolve_path(args.file);
    log::debug!("Resolved path: {:?}", &file_path);

    let tasks = storage::load(&file_path)?;
    let mut manager = TaskManager::new(tasks);

    match args.command {
        cli::Command::Add { description } => {
            let task = manager.add(description)?;
            println!("Task added:\t {:?}", task);
        }
        cli::Command::List { completed, pending } => {
            let task_list = manager.list(completed, pending).collect::<Vec<&Task>>();

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
    storage::save(&tasks, &file_path)?;

    Ok(())
}

fn init_logger(level: LevelFilter) {
    env_logger::Builder::new()
        .filter(Some("task"), level)
        .init()
}
