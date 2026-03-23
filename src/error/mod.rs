use std::io::{Error as IoError, ErrorKind};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task with ID {0} not found.")]
    NotFound(u32),
    #[error("Task with ID {0} already completed.")]
    AlreadyCompleted(u32),
    #[error("Missing a description for the task.")]
    EmptyDescription,
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Cannot connect to the storage for retrieving tasks.")]
    ConnectionError,
    #[error("An unexpected error occurred while retrieving the tasks.")]
    UnexpectedError,
    #[error("Failed to parse the tasks")]
    ParsingError,
}

impl From<IoError> for StorageError {
    fn from(value: IoError) -> Self {
        match value.kind() {
            ErrorKind::NotFound => StorageError::ConnectionError,
            _ => StorageError::UnexpectedError,
        }
    }
}
