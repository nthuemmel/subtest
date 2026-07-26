use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TaskStatus {
    Pending,
    Completed,
    Cancelled,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TaskStatus::Pending => "Pending",
            TaskStatus::Completed => "Completed",
            TaskStatus::Cancelled => "Cancelled",
        };
        write!(f, "{}", label)
    }
}

pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: TaskStatus,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] ({}) {}", self.id, self.status, self.description)
    }
}

#[derive(Debug)]
pub enum TodoError {
    TaskNotFound(u32),
    InvalidTransition {
        id: u32,
        from: TaskStatus,
        to: TaskStatus,
    },
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoError::TaskNotFound(id) => write!(f, "task {} not found", id),
            TodoError::InvalidTransition { id, from, to } => {
                write!(f, "cannot transition task {} from {} to {}", id, from, to)
            }
        }
    }
}

impl std::error::Error for TodoError {}

pub struct TodoList {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, description: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.tasks.push(Task {
            id,
            description: description.into(),
            status: TaskStatus::Pending,
        });
        id
    }

    /// Marks a task as completed.
    ///
    /// Fails if the task doesn't exist or is already completed/cancelled.
    pub fn complete(&mut self, id: u32) -> Result<(), TodoError> {
        self.set_status(id, TaskStatus::Completed)
    }

    /// Marks a task as cancelled.
    ///
    /// Fails if the task doesn't exist or is already completed/cancelled.
    pub fn cancel(&mut self, id: u32) -> Result<(), TodoError> {
        self.set_status(id, TaskStatus::Cancelled)
    }

    /// Internal helper: transitions a task to `new_status`, only if it is
    /// currently `Pending`.
    fn set_status(&mut self, id: u32, new_status: TaskStatus) -> Result<(), TodoError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.id == id)
            .ok_or(TodoError::TaskNotFound(id))?;

        if task.status != TaskStatus::Pending {
            return Err(TodoError::InvalidTransition {
                id,
                from: task.status,
                to: new_status,
            });
        }

        task.status = new_status;
        Ok(())
    }

    /// Returns a reference to a task by id, if it exists.
    pub fn get(&self, id: u32) -> Option<&Task> {
        self.tasks.iter().find(|t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use subtest::subtest;

    #[subtest]
    #[test]
    fn add_creates_pending_task() {
        let mut list = TodoList::new();
        let id = list.add("Buy milk");

        #[subtest]
        fn complete_marks_task_completed() {
            list.complete(id).unwrap();
            assert_eq!(list.get(id).unwrap().status, TaskStatus::Completed);
        }

        #[subtest]
        fn cancel_marks_task_cancelled() {
            list.cancel(id).unwrap();
            assert_eq!(list.get(id).unwrap().status, TaskStatus::Cancelled);

            #[subtest]
            fn cannot_complete_already_cancelled_task() {
                let err = list.complete(id).unwrap_err();
                assert!(matches!(err, TodoError::InvalidTransition { .. }));
            }
        }

        let task = list.get(id).unwrap();
        assert_eq!(task.status, TaskStatus::Pending);
    }
}
