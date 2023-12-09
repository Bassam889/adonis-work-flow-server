use axum::http::StatusCode;
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::utils::app_error::AppError;

pub mod create_user;
pub mod guard;
pub mod login;
pub mod logout;

// #[derive(Serialize, Deserialize)]
// pub struct ResponseDataUser {
//     data: ResponseUser,
// }

#[derive(Deserialize)]
pub struct RequestSignUp {
    username: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseSignUp {
    id: Uuid,
    username: String,
    email: String,
    token: String,
}

#[derive(Deserialize)]
pub struct RequestLogin {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseLogin {
    id: Uuid,
    username: String,
    email: String,
    token: String,
}

pub fn hash_password(password: String) -> Result<String, AppError> {
    bcrypt::hash(password, 10)
        .map_err(|_error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))
}

pub fn verify_password(password: String, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|_error| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))
}
