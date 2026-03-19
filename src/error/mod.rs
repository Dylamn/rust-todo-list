use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task with ID {0} not found.")]
    NotFound(u32),
    #[error("Task with ID {0} already completed.")]
    AlreadyCompleted(u32),
    #[error("Missing a description for the task.")]
    EmptyDescription
}
