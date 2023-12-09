use axum::extract::Query;
use axum::{http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::database::tasks::{self, Entity as Tasks};

use super::ResponseTask;

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    is_completed: Option<bool>,
}

pub async fn get_all_tasks(
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<GetTasksQueryParams>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut is_completed_filter = Condition::all();
    if let Some(is_completed) = query_params.is_completed {
        is_completed_filter = is_completed_filter.add(tasks::Column::IsCompleted.eq(is_completed));
    }
    let tasks = Tasks::find()
        .filter(is_completed_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_err| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|task| ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            is_completed: task.is_completed,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        })
        .collect();

    Ok(Json(tasks))
}
