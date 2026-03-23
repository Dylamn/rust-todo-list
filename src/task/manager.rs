use super::Task;
use crate::error::TaskError;
use chrono::Utc;
use log::debug;

pub struct TaskManager {
    next_id: u32,
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new(tasks: Vec<Task>) -> Self {
        let next_id = tasks
            .iter()
            .map(|t| t.id)
            .max()
            .map(|ident| ident + 1)
            .unwrap_or(1u32);

        debug!("Next task ID: {next_id}");

        Self { next_id, tasks }
    }

    pub fn add(&mut self, description: String) -> Result<Task, TaskError> {
        debug!("Adding new task...");
        if description.is_empty() {
            return Err(TaskError::EmptyDescription);
        }

        let task = Task::new(self.next_id, description);
        self.tasks.push(task.clone());
        self.next_id += 1;
        println!("Incrementing ID... next value: {}", self.next_id);

        Ok(task)
    }

    pub fn list(&self, completed: bool, pending: bool) -> impl Iterator<Item = &Task> {
        self.tasks.iter().filter(move |t| {
            if completed && pending {
                // The user provided both, return all.
                true
            } else if completed {
                t.completed_at.is_some()
            } else if pending {
                t.completed_at.is_none()
            } else {
                // The user doesn't provide any argument, return the complete list.
                true
            }
        })
    }

    pub fn mark_done(&mut self, id: u32) -> Result<(), TaskError> {
        debug!("Trying to mark task {id} as done.");
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;

        if task.completed_at.is_some() {
            return Err(TaskError::AlreadyCompleted(id));
        }

        task.completed_at = Some(Utc::now());
        debug!("Task {id} marked as done.");

        Ok(())
    }

    pub fn remove(&mut self, id: u32) -> Result<(), TaskError> {
        let index = self
            .tasks
            .iter()
            .position(|t| t.id == id)
            .ok_or(TaskError::NotFound(id))?;

        self.tasks.remove(index);
        debug!("Removed task {id} from the list.");

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::Task;

    fn create_manager() -> TaskManager {
        TaskManager::new(vec![])
    }

    fn create_task(id: u32, description: &str, completed: bool) -> Task {
        let mut task = Task::new(id, description.to_string());
        if completed {
            task.completed_at = Some(Utc::now());
        }
        task
    }

    // =========================
    // ADD
    // =========================

    #[test]
    fn should_add_task() {
        let mut manager = create_manager();
        let task_description = "Test task";

        let result = manager.add(task_description.to_string());

        assert!(result.is_ok());
        assert_eq!(manager.tasks.len(), 1);

        let task = manager.tasks.get(0).unwrap();
        assert_eq!(task.description, task_description.to_string());
    }

    #[test]
    fn should_increment_id_when_adding_tasks() {
        let mut manager = create_manager();
        let id1 = manager.next_id;
        let t1 = manager.add("Task 1".to_string()).unwrap();
        let id2 = manager.next_id;
        let t2 = manager.add("Task 2".to_string()).unwrap();

        assert_eq!(t1.id + 1, t2.id);
        assert_eq!(id1, t1.id);
        assert_eq!(id2, t2.id);
    }

    #[test]
    fn should_fail_when_description_is_empty() {
        let mut manager = create_manager();

        let result = manager.add("".to_string());

        assert!(matches!(result, Err(TaskError::EmptyDescription)));
    }

    // =========================
    // LIST
    // =========================

    #[test]
    fn should_list_all_tasks_when_no_filter() {
        let tasks = vec![
            create_task(1, "A", false),
            create_task(2, "B", true),
        ];

        let manager = TaskManager::new(tasks);

        let result: Vec<_> = manager.list(false, false).collect();

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn should_filter_completed_tasks() {
        let tasks = vec![
            create_task(1, "A", false),
            create_task(2, "B", true),
        ];

        let manager = TaskManager::new(tasks);

        let result: Vec<_> = manager.list(true, false).collect();

        assert_eq!(result.len(), 1);
        assert!(result[0].completed_at.is_some());
    }

    #[test]
    fn should_filter_pending_tasks() {
        let tasks = vec![
            create_task(1, "A", false),
            create_task(2, "B", true),
        ];

        let manager = TaskManager::new(tasks);

        let result: Vec<_> = manager.list(false, true).collect();

        assert_eq!(result.len(), 1);
        assert!(result[0].completed_at.is_none());
    }

    // =========================
    // MARK DONE
    // =========================

    #[test]
    fn should_mark_task_as_done() {
        let tasks = vec![create_task(1, "A", false)];
        let mut manager = TaskManager::new(tasks);

        let result = manager.mark_done(1);

        assert!(result.is_ok());

        let task = manager.tasks.iter().find(|t| t.id == 1).unwrap();
        assert!(task.completed_at.is_some());
    }

    #[test]
    fn should_fail_when_task_not_found() {
        let mut manager = create_manager();

        let result = manager.mark_done(42);

        assert!(matches!(result, Err(TaskError::NotFound(42))));
    }

    #[test]
    fn should_fail_if_task_already_completed() {
        let tasks = vec![create_task(1, "A", true)];
        let mut manager = TaskManager::new(tasks);

        let result = manager.mark_done(1);

        assert!(matches!(result, Err(TaskError::AlreadyCompleted(1))));
    }

    // =========================
    // REMOVE
    // =========================

    #[test]
    fn should_remove_task() {
        let tasks = vec![create_task(1, "A", false)];
        let mut manager = TaskManager::new(tasks);

        let result = manager.remove(1);

        assert!(result.is_ok());
        assert!(manager.tasks.is_empty());
    }

    #[test]
    fn should_fail_when_removing_unknown_task() {
        let mut manager = create_manager();

        let result = manager.remove(42);

        assert!(matches!(result, Err(TaskError::NotFound(42))));
    }
}