use crate::database::users::Model;
use axum::{http::StatusCode, Extension};

use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
