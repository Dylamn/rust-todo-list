use crate::error::StorageError;
use crate::task::Task;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, ErrorKind};
use std::path::Path;

pub fn load(path: &Path) -> Result<Vec<Task>, StorageError> {
    let file = open_file(path)?;
    let reader = BufReader::new(file);

    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new());

    Ok(tasks)
}

fn create_file(path: &Path) -> Result<File, StorageError> {
    let new_file = File::create(path)?;
    Ok(new_file)
}

fn open_file(path: &Path) -> Result<File, StorageError> {
    let open_result = File::open(path);

    match open_result {
        Ok(file) => Ok(file),
        Err(e) if e.kind() == ErrorKind::NotFound => create_file(path),
        Err(_) => Err(StorageError::UnexpectedError),
    }
}

pub fn save(tasks: &Vec<Task>, path: &Path) -> Result<(), StorageError> {
    let storage_file = OpenOptions::new()
        .read(false)
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    serde_json::to_writer(storage_file, tasks).map_err(|_| StorageError::UnexpectedError)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    fn create_path() -> std::path::PathBuf {
        let dir = tempdir().unwrap();
        dir.path().join("tasks.json")
    }

    fn create_task(id: u32, description: &str) -> Task {
        Task::new(id, description.to_string())
    }

    // =========================
    // LOAD
    // =========================

    #[test]
    fn should_return_empty_vec_if_file_does_not_exist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        let result = load(&path).unwrap();

        assert!(result.is_empty());
    }

    #[test]
    fn should_load_tasks_from_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        let tasks = vec![
            create_task(1, "A"),
            create_task(2, "B"),
        ];

        save(&tasks, &path).unwrap();

        let loaded = load(&path).unwrap();

        assert_eq!(loaded.len(), 2);
    }

    // =========================
    // SAVE
    // =========================

    #[test]
    fn should_create_file_when_saving() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        let tasks = vec![create_task(1, "A")];

        save(&tasks, &path).unwrap();

        assert!(path.exists());
    }

    #[test]
    fn should_overwrite_existing_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        let tasks1 = vec![create_task(1, "Old")];
        save(&tasks1, &path).unwrap();

        let tasks2 = vec![create_task(2, "New")];
        save(&tasks2, &path).unwrap();

        let loaded = load(&path).unwrap();

        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].description, "New");
    }

    // =========================
    // INTERNALS
    // =========================

    #[test]
    fn open_file_should_create_file_if_not_exists() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        let file = open_file(&path).unwrap();

        assert!(path.exists());
        drop(file);
    }

    #[test]
    fn create_file_should_create_empty_file() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("tasks.json");

        create_file(&path).unwrap();

        assert!(path.exists());

        let metadata = fs::metadata(&path).unwrap();
        assert_eq!(metadata.len(), 0);
    }
}
