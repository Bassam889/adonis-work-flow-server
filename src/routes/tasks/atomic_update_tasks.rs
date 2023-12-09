use axum::{extract::Path, Extension, Json, http::StatusCode};
use sea_orm::{prelude::Uuid, DatabaseConnection, Set, EntityTrait, ColumnTrait, QueryFilter};

use crate::routes::tasks::RequestTask;
use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;

pub async fn atomic_update_tasks(
    Path(task_id): Path<Uuid>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>
) -> Result<(), StatusCode> {
   let update_task = tasks::ActiveModel {
        id: Set(task_id),
        title: Set(request_task.title),
        description: Set(request_task.description),
        is_completed: Set(request_task.is_completed),
        created_at: Set(request_task.created_at),
        completed_at: Set(request_task.completed_at),
        deleted_at: Set(request_task.deleted_at),
        user_id: Set(request_task.user_id), 
    };

    Tasks::update(update_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
