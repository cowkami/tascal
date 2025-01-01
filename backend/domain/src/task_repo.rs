use anyhow::Result;
use async_trait::async_trait;

use crate::task::{Task, TaskStatus};

#[async_trait]
pub trait TaskRepo {
    async fn save(&self, task: Task) -> Result<Task>;
    async fn list_by_assignee(&self, assignee: String) -> Result<Vec<Task>>;
    async fn list_by_status(&self, status: TaskStatus) -> Result<Vec<Task>>;
    async fn update(&self, id: String, task: Task) -> Result<Task>;
    async fn delete(&self, id: String) -> Result<Task>;
}
