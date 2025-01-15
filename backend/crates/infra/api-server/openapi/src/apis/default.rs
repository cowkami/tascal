use async_trait::async_trait;
use axum::extract::*;
use axum_extra::extract::{CookieJar, Multipart};
use bytes::Bytes;
use http::Method;
use serde::{Deserialize, Serialize};

use crate::{models, types::*};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum HealthGetResponse {
    /// 成功
    Status200
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksGetResponse {
    /// 成功
    Status200
    (Vec<models::Task>)
    ,
    /// クエリが不正
    Status400
    (String)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
#[allow(clippy::large_enum_variant)]
pub enum TasksPostResponse {
    /// 作成成功
    Status201
    (models::Task)
}


/// Default
#[async_trait]
#[allow(clippy::ptr_arg)]
pub trait Default {
    /// サーバーのヘルスチェック.
    ///
    /// HealthGet - GET /api/v1/health
    async fn health_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
    ) -> Result<HealthGetResponse, ()>;

    /// タスクの一覧を取得.
    ///
    /// TasksGet - GET /api/v1/tasks
    async fn tasks_get(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<models::TasksGetRequest>,
    ) -> Result<TasksGetResponse, ()>;

    /// タスクを新規作成.
    ///
    /// TasksPost - POST /api/v1/tasks
    async fn tasks_post(
    &self,
    method: Method,
    host: Host,
    cookies: CookieJar,
            body: Option<models::Task>,
    ) -> Result<TasksPostResponse, ()>;
}
