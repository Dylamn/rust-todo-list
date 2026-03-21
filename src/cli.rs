use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "task")]
#[command(about = "Simple CLI task manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Adk a task to the list
    Add {
        description: String,
    },

    /// List all the tasks
    List {
        /// Show only completed tasks
        #[arg(long)]
        completed: bool,

        /// Show only pending tasks
        #[arg(long)]
        pending: bool,
    },

    /// Mark a task as done
    Done {
        id: u32,
    },


    /// Remove a task from the list
    Remove {
        id: u32,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}