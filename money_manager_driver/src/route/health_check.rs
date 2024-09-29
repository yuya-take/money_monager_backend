use std::sync::Arc;

use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;

use crate::context::error::ApiError;
use crate::module::{Modules, ModulesExt};

#[utoipa::path(
    get,
    path = "/hc",
    responses(
        (status = 200, description = "OK"),
    ),
    tag = "Health Check"
)]
pub async fn hc_hello() -> impl IntoResponse {
    // レスポンスを返す
    tracing::info!("hc_hello");
    "Hello, World!"
}

#[utoipa::path(
    get,
    path = "/hc/dynamodb",
    responses(
        (status = 200, description = "OK"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "Health Check"
)]
pub async fn hc_dynamodb(
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("hc_dynamodb");
    module
        .health_check_use_case()
        .diagnose_dynamodb_conn()
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| {
            tracing::error!("Failed to diagnose mysql conn: {:?}", e);
            ApiError::DatabaseConnectionError()
        })
}
