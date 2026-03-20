use crate::error::StorageError;
use crate::task::Task;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, ErrorKind};

// TODO: define file path (configurable later)
const FILE_PATH: &str = "tasks.json";

pub fn load() -> Result<Vec<Task>, StorageError> {
    let file = open_file()?;
    let reader = BufReader::new(file);

    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());

    Ok(tasks)
}

fn create_file() -> Result<File, StorageError> {
    let new_file = File::create(FILE_PATH)?;

    Ok(new_file)
}

fn open_file() -> Result<File, StorageError> {
    let open_result = File::open(FILE_PATH);

    match open_result {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::NotFound => create_file(),
        Err(_) => Err(StorageError::UnexpectedError),
    }
}

pub fn save(tasks: &Vec<Task>) -> Result<(), StorageError> {
    let storage_file = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .create(true)
        .open(FILE_PATH)?;

    serde_json::to_writer(storage_file, tasks).map_err(|_| StorageError::UnexpectedError)?;

    Ok(())
}
