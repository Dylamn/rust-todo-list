use super::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add(&mut self, description: String) {
        todo!("implement TaskManager.add")
    }

    pub fn list(&self) -> &Vec<Task> {
        todo!("Implement TaskManager.list")
    }

    pub fn mark_done(&mut self, id: u64) -> () {
        todo!("implement TaskManager.mark_done")
    }

    pub fn remove(&mut self, id: u64) -> () {
        todo!("implement TaskManager.remove")
    }
}