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
    Add {
        description: String,
    },

    List {
        /// Show only completed tasks
        #[arg(long)]
        completed: bool,

        /// Show only pending tasks
        #[arg(long)]
        pending: bool,
    },

    Done {
        id: u32,
    },

    Remove {
        id: u32,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}