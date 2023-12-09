use axum::{extract::Path, Extension, Json, http::StatusCode};
use sea_orm::{
    prelude::{Uuid, DateTimeWithTimeZone},
    DatabaseConnection, Set, EntityTrait, ColumnTrait, QueryFilter, IntoActiveModel
};
use serde::Deserialize;
use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub is_completed: Option<Option<bool>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub created_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub completed_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub deleted_at: Option<Option<DateTimeWithTimeZone>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option"
    )]
    pub user_id: Option<Option<Uuid>>,
}

pub async fn partial_update_tasks(
    Path(task_id): Path<Uuid>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>
) -> Result<(), StatusCode> {
    let mut task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? {
            task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(is_completed) = request_task.is_completed {
        task.is_completed = Set(is_completed)
    }

    if let Some(title) = request_task.title {
        task.title = Set(title)
    }

    if let Some(description) = request_task.description {
        task.description = Set(description)
    }

    if let Some(completed_at) = request_task.completed_at {
        task.completed_at = Set(completed_at)
    }
    
    if let Some(created_at) = request_task.created_at {
        task.created_at = Set(created_at)
    }

    if let Some(deleted_at) = request_task.deleted_at {
        task.deleted_at = Set(deleted_at)
    }

    Tasks::update(task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
