use anyhow::Result;
use async_trait::async_trait;
use sqlx::postgres::PgPool;

use domain::task::{Task, TaskId, TaskStatus};
use domain::task_repo::TaskRepo;

#[derive(Clone)]
pub struct TaskRepoImpl {
    pool: PgPool,
}

impl TaskRepoImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepo for TaskRepoImpl {
    async fn save(&self, task: Task) -> Result<Task> {
        todo!();
    }

    async fn list_by_id_list(&self, id_list: Vec<TaskId>) -> Result<Vec<Task>> {
        todo!();
    }

    async fn list_by_assignee(&self, assignee: String) -> Result<Vec<Task>> {
        todo!();
    }

    async fn list_by_status(&self, status: TaskStatus) -> Result<Vec<Task>> {
        todo!();
    }

    async fn update(&self, id: TaskId, task: Task) -> Result<Task> {
        todo!();
    }

    async fn delete(&self, id: TaskId) -> Result<Task> {
        todo!()
    }
}
