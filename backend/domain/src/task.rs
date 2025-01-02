use anyhow::{Context, Result};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Task {
    id: TaskId,
    title: String,
    description: String,
    assignee: String,
    status: TaskStatus,
}

#[derive(Debug, Clone)]
pub struct TaskId(Uuid);

#[derive(Debug, Clone)]
pub enum TaskStatus {
    ToDo,
    InProgress,
    Done,
}

impl Task {
    pub fn new(title: String, description: String, assignee: String) -> Self {
        Self {
            id: TaskId::new(),
            title,
            description,
            assignee,
            status: TaskStatus::ToDo,
        }
    }
}

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl From<TaskId> for String {
    fn from(id: TaskId) -> Self {
        id.0.to_string()
    }
}

impl TryFrom<String> for TaskId {
    type Error = anyhow::Error;

    fn try_from(id: String) -> Result<TaskId> {
        let id = Uuid::parse_str(&id)
            .with_context(|| format!("Failed to parse task id from string: {}", id))?;
        Ok(TaskId(id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_id_try_from() {
        let task_id = TaskId::try_from("1234567890".to_string());
        assert!(task_id.is_err());

        let task_id = TaskId::try_from("67e55044-10b1-426f-9247-bb680e5fe0c8".to_string());
        assert!(task_id.is_ok());
    }
}
