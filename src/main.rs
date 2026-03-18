mod cli;
pub mod error;
pub mod storage;
pub mod task;

fn main() {
    let args = cli::parse();

    match args.command {
        cli::Command::Add { description } => {
            println!("Add: {}", description);
        }
        cli::Command::List { completed, pending } => {
            println!("Listing tasks...");
        }
        cli::Command::Done { id } => {
            println!("Marking task {} as done", id);
        }
        cli::Command::Remove { id } => {
            println!("Remove task with id: {}", id);
        }
        _ => {
            println!("Command not implemented");
        }
    }

    // TODO: Save tasks back to storage
}
