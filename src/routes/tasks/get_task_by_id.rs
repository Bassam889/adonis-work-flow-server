use axum::{extract::Path, http::StatusCode, Extension, Json};
use sea_orm::{prelude::Uuid, DatabaseConnection};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::database::tasks::{self, Entity as Tasks};

use super::ResponseTask;

pub async fn get_task_by_id(
    Path(task_id): Path<Uuid>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            description: task.description,
            is_completed: task.is_completed,
            deleted_at: task.deleted_at,
            user_id: task.user_id,
        }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
