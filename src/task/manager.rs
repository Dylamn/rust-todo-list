use super::Task;
use crate::error::TaskError;
use chrono::Utc;

pub struct TaskManager {
    next_id: u32,
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            // TODO: When implementing persistence,
            //  the `next_id` should start after the higher value in the persisted elements.
            next_id: 1,
            tasks: Vec::new(),
        }
    }

    pub fn add(&mut self, description: String) -> Result<Task, TaskError> {
        if description.is_empty() {
            return Err(TaskError::EmptyDescription);
        }

        let task = Task::new(self.next_id, description);
        self.tasks.push(task.clone());
        self.next_id += 1;

        Ok(task)
    }

    pub fn list(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn mark_done(&mut self, id: u32) -> Result<(), TaskError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;

        if task.completed_at.is_some() {
            return Err(TaskError::AlreadyCompleted(id));
        }

        task.completed_at = Some(Utc::now());

        Ok(())
    }

    pub fn remove(&mut self, id: u32) -> Result<(), TaskError> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;

        self.tasks.remove(index);

        Ok(())
    }
}
