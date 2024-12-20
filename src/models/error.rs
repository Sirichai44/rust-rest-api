use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use mongodb::error::Error;
use serde_json::json;

pub struct ErrorResponse {
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(json!(
                {
                    "error": self.message,
                }
            )),
        )
            .into_response()
    }
}

pub trait IntoErrorResponse {
    fn error(&self) -> ErrorResponse;
}

pub enum APIError {
    AddPersonError(Error),
    GetPersonError(Error),
    UpdatePersonError(Error),
    DeletePersonError(Error),
    PersonAlreadyExistsError(String, String),
}

impl IntoErrorResponse for APIError {
    fn error(&self) -> ErrorResponse {
        match self {
            Self::AddPersonError(_) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to add a person".to_string(),
            },
            Self::GetPersonError(_) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to get a person".to_string(),
            },
            Self::UpdatePersonError(_) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to update a person".to_string(),
            },
            Self::DeletePersonError(_) => ErrorResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Failed to delete a person".to_string(),
            },
            Self::PersonAlreadyExistsError(name, email) => ErrorResponse {
                status: StatusCode::BAD_REQUEST,
                message: format!("Person {} with email {} already exists", name, email),
            },
        }
    }
}
