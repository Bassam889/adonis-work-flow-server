use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::convert::Into;

pub struct AppError {
    statuscode: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(statuscode: StatusCode, message: impl Into<String>) -> Self {
        Self {
            statuscode,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            self.statuscode,
            Json(ResponseMessage {
                message: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}
