use anyhow::{Context, Result};

use domain::task::{Task, TaskId, TaskStatus};
use domain::task_repo::{ProvideTaskRepo, TaskRepo};

pub async fn create_task<T>(ctx: &T, task: Task) -> Result<Task>
where
    T: ProvideTaskRepo,
{
    let task_repo = ProvideTaskRepo::provide(ctx);

    let saved_task = task_repo
        .save(task.clone())
        .await
        .with_context(|| format!("Failed to save task: {:?}", task))?;

    Ok(saved_task)
}

#[derive(Debug, Clone)]
pub enum ListTaskQuery {
    ByIdList(Vec<TaskId>),
    ByAssignee(String),
    ByStatus(TaskStatus),
}

pub async fn list_task<T>(ctx: &T, query: ListTaskQuery) -> Result<Vec<Task>>
where
    T: ProvideTaskRepo,
{
    let task_repo = ProvideTaskRepo::provide(ctx);

    let task_list = match query.clone() {
        ListTaskQuery::ByIdList(id_list) => task_repo.list_by_id_list(id_list),
        ListTaskQuery::ByAssignee(assignee) => task_repo.list_by_assignee(assignee),
        ListTaskQuery::ByStatus(status) => task_repo.list_by_status(status),
    }
    .await
    .with_context(|| format!("Failed to list tasks: {:?}", query))?;

    Ok(task_list)
}

pub async fn update_task<T>(ctx: &T, id: TaskId, task: Task) -> Result<Task>
where
    T: ProvideTaskRepo,
{
    let task_repo = ProvideTaskRepo::provide(ctx);

    let updated_task = task_repo
        .update(id, task.clone())
        .await
        .with_context(|| format!("Failed to update task: {:?}", task))?;

    Ok(updated_task)
}

pub async fn delete_task<T>(ctx: &T, id: TaskId) -> Result<Task>
where
    T: ProvideTaskRepo,
{
    let task_repo = ProvideTaskRepo::provide(ctx);

    let deleted_task = task_repo
        .delete(id.clone())
        .await
        .with_context(|| format!("Failed to delete task: {:?}", id))?;

    Ok(deleted_task)
}
