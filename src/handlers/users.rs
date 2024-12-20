use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{models::persons::Person, usecases::users::UsersUseCase};

#[axum_macros::debug_handler]
pub async fn users_adding(
    Extension(users_usecase): Extension<Arc<UsersUseCase>>,
    Json(payload): Json<Person>,
) -> impl IntoResponse {
    let result = match users_usecase.adding(payload.name, payload.email).await {
        Ok(inserted_id) => inserted_id,
        Err(e) => return e.error().into_response(),
    };

    (StatusCode::CREATED, Json(result)).into_response()
}
