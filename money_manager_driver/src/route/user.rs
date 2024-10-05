use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::context::error::ApiError;
use crate::model::user::UserResponse;
use crate::module::{Modules, ModulesExt};

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    responses(
        (status = 200, description = "OK"),
    ),
    tag = "User"
)]
pub async fn get_user(
    Path(user_id): Path<String>,
    Extension(module): Extension<Arc<Modules>>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::info!("get user: {}", user_id);
    module
        .user_use_case()
        .get_user(user_id)
        .await
        .map(|user_view| {
            let user_response = UserResponse::from(user_view);
            (StatusCode::OK, Json(user_response))
        })
        .map_err(|e| {
            tracing::error!("Failed to get user: {:?}", e);
            ApiError::NotFoundError("User not found".to_string())
        })
}
