mod tasks;

use anyhow::Result;
use axum::async_trait;
use axum::extract::Host;
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
struct ApiImpl<T>(T);

#[async_trait]
impl<T> Default for ApiImpl<T>
where
    T: ProvideTaskRepo + Send + Sync + Clone,
{
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
            .clone()
            .ok_or(anyhow::anyhow!("Invalid request"))
            .and_then(TasksGetRequestWrapper::try_from)
            .and_then(ListTaskQuery::try_from)
            .expect(format!("Faled to parse request: {:?}", request).as_str());

        println!("query: {:?}", query);

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
    let app = openapi::server::new(ApiImpl(context.clone())).layer(cors);

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

impl<T> AsRef<ApiImpl<T>> for ApiImpl<T>
where
    T: Send + Sync + Clone + ProvideTaskRepo,
{
    fn as_ref(&self) -> &ApiImpl<T> {
        self
    }
}

impl<T> Clone for ApiImpl<T>
where
    T: Send + Sync + Clone + ProvideTaskRepo,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
