use crate::database::users::{self, Entity as Users};
use crate::utils::app_error::AppError;
use crate::utils::jwt::is_token_valid;
use axum::{
    headers::{authorization::Bearer, Authorization, HeaderMapExt},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

pub async fn guard<T>(mut request: Request<T>, next: Next<T>) -> Result<Response, AppError> {
    let token = request
        .headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(AppError::new(StatusCode::BAD_REQUEST, "Missing token"))?
        .token()
        .to_owned();

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
        ))?;

    let user = Users::find()
        .filter(users::Column::Token.eq(Some(token.clone())))
        .one(database)
        .await
        .map_err(|_error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        })?;
    is_token_valid(&token)?;

    let Some(user) = user else { return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized, please log in or create an account")) };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
