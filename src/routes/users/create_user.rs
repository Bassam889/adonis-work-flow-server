use crate::utils::jwt::create_jwt;
use crate::{database::users, utils::app_error::AppError};

use super::{hash_password, RequestSignUp, ResponseSignUp};
use axum::{http::StatusCode, Json, Extension};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn create_user(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestSignUp>,
) -> Result<Json<ResponseSignUp>, AppError> {
    let token = create_jwt()?;
    let new_user = users::ActiveModel {
        username: Set(request_user.username),
        email: Set(request_user.email),
        password: Set(hash_password(request_user.password)?),
        token: Set(Some(token)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_error| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Something went wrong while creating the user, please try again",
        )
    })?;

    Ok(Json(ResponseSignUp {
        id: new_user.id.unwrap(),
        username: new_user.username.unwrap(),
        email: new_user.email.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
}
