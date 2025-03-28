use anyhow::{Context, Result};
use env_logger;

use db::TaskRepoImpl;
use domain::task_repo::ProvideTaskRepo;
use server;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting app...");

    let context = AppContext::new().await?;
    server::run(context).await?;

    log::info!("Server stopped");
    Ok(())
}

#[derive(Clone)]
struct AppContext {
    task_repo: TaskRepoImpl,
}

impl AppContext {
    async fn new() -> Result<Self> {
        log::info!("Connecting to database...");
        let db_connection = db::get_connection()
            .await
            .with_context(|| "Failed to connect to database")?;
        log::info!("Connected to database");

        Ok(Self {
            task_repo: TaskRepoImpl::new(db_connection),
        })
    }
}

impl ProvideTaskRepo for AppContext {
    type Repository = TaskRepoImpl;

    fn provide(&self) -> &Self::Repository {
        &self.task_repo
    }
}
