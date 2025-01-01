use anyhow::Result;
use axum::async_trait;
use axum::extract::Host;
use axum_extra::extract::CookieJar;
use http::Method;

use openapi::apis::default::Default;
use openapi::apis::default::{HealthGetResponse, TasksGetResponse, TasksPostResponse};

struct ApiImpl;

#[async_trait]
impl Default for ApiImpl {
    async fn health_get(&self, _: Method, _: Host, _: CookieJar) -> Result<HealthGetResponse, ()> {
        Ok(HealthGetResponse::Status200("ok".to_string()))
    }

    async fn tasks_get(&self, _: Method, _: Host, _: CookieJar) -> Result<TasksGetResponse, ()> {
        Ok(TasksGetResponse::Status200(vec![]))
    }

    async fn tasks_post(
        &self,
        _: Method,
        _: Host,
        _: CookieJar,
        _: Option<openapi::models::Task>,
    ) -> Result<TasksPostResponse, ()> {
        Ok(TasksPostResponse::Status201)
    }
}

impl AsRef<ApiImpl> for ApiImpl {
    fn as_ref(&self) -> &ApiImpl {
        self
    }
}

impl Clone for ApiImpl {
    fn clone(&self) -> Self {
        Self
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting server");

    let api_impl = ApiImpl;
    // build our application with a single route
    let app = openapi::server::new(api_impl);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");
    axum::serve(listener, app).await.expect("Failed to serve");

    log::info!("Server stopped");
    Ok(())
}
