use axum::{
    http::StatusCode,
    Json,
    extract::Path,
};

use crate::{
    models::User,
    services::user_service,
};

pub async fn get_user_handler(Path(user_id): Path<i64>) -> Result<Json<User>, StatusCode> {
    user_service::get_user(user_id)
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
