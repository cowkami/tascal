mod tasks;

use std::sync::Arc;

use anyhow::Result;
use axum::async_trait;
use axum::extract::Host;
use axum::Extension;
use axum_extra::extract::CookieJar;
use http::HeaderValue;
use http::Method;
use tower_http::cors::CorsLayer;

use domain::task_repo::ProvideTaskRepo;
use openapi::apis::default::Default;
use openapi::apis::default::{HealthGetResponse, TasksGetResponse, TasksPostResponse};
use usecase::task::ListTaskQuery;

use crate::tasks::*;

#[derive(Debug)]
struct ApiImpl;

#[async_trait]
impl Default for ApiImpl {
    async fn health_get(&self, _: Method, _: Host, _: CookieJar) -> Result<HealthGetResponse, ()> {
        Ok(HealthGetResponse::Status200("ok".to_string()))
    }

    async fn tasks_get(
        &self,
        _: Method,
        _: Host,
        _: CookieJar,
        request: Option<openapi::models::TasksGetRequest>,
    ) -> Result<TasksGetResponse, ()> {
        let query = request
            .ok_or(anyhow::anyhow!("Invalid request"))
            .and_then(TasksGetRequestWrapper::try_from)
            .and_then(ListTaskQuery::try_from)
            .expect("Invalid request");

        println!("query: {:?}", query);

        println!("api impl: {:?}", self);

        todo!()
    }

    async fn tasks_post(
        &self,
        _: Method,
        _: Host,
        _: CookieJar,
        _: Option<openapi::models::Task>,
    ) -> Result<TasksPostResponse, ()> {
        todo!()
    }
}

pub async fn run<T>(context: T) -> Result<()>
where
    T: 'static + Clone + Send + Sync + ProvideTaskRepo,
{
    log::info!("Starting api server...");

    // Arc is used to allow the context to be shared across threads
    let context = Arc::new(context);

    // CORS for development
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(vec![
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ORIGIN,
        ]);

    // build our application with a single route
    let app = openapi::server::new(ApiImpl)
        .layer(Extension(context))
        .layer(cors);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    axum::serve(listener, app)
        .await
        .expect("Failed to start api server");

    log::info!("Api server stopped");
    Ok(())
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
