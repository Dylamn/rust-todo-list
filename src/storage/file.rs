use crate::task::Task;

// TODO: define file path (configurable later)
const FILE_PATH: &str = "tasks.json";


pub fn load() -> Vec<Task> {
    // TODO: handle file not existing
    // TODO: read file
    // TODO: deserialize JSON
    todo!("Implement file.load")
}

pub fn save(tasks: &Vec<Task>) {
    // TODO: serialize JSON
    // TODO: write file
    todo!("Implement file.save");
}