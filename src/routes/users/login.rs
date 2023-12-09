use crate::database::users::Entity as Users;
use crate::utils::app_error::AppError;
use axum::{http::StatusCode, Json, Extension};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use super::{verify_password, RequestLogin, ResponseLogin};

use crate::database::users;
use crate::utils::jwt::create_jwt;

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestLogin>,
) -> Result<Json<ResponseLogin>, AppError> {
    let jwt = create_jwt()?;
    let db_user = Users::find()
        .filter(users::Column::Email.eq(request_user.email))
        .one(&database)
        .await
        .map_err(|_error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, please try again",
            )
        })?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "password is incorrect, please try again",
            ));
        }

        let new_token = jwt;
        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));

        let saved_user = user.save(&database).await.map_err(|_error| {
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "An error occured while login in, please try again",
            )
        })?;

        Ok(Json(ResponseLogin {
            id: saved_user.id.unwrap(),
            username: saved_user.username.unwrap(),
            email: saved_user.email.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
    } else {
        Err(AppError::new(StatusCode::NOT_FOUND, "User's not found"))
    }
}
