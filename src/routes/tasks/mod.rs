pub mod atomic_update_tasks;
pub mod create_task;
pub mod delete_task;
pub mod get_all_tasks;
pub mod get_task_by_id;
pub mod partial_update_tasks;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use sea_orm::prelude::{DateTimeWithTimeZone, Uuid};

#[derive(Serialize)]
pub struct ResponseTask {
    id: Uuid,
    title: String,
    description: Option<String>,
    is_completed: Option<bool>,
    deleted_at: Option<DateTime<FixedOffset>>,
    user_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct RequestTask {
    pub id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub is_completed: Option<bool>,
    pub created_at: Option<DateTimeWithTimeZone>,
    pub completed_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<Uuid>,
}
