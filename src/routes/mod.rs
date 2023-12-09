mod tasks;
mod users;

use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

use axum::http::Method;
use axum::Extension;
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use tasks::atomic_update_tasks::atomic_update_tasks;
use tasks::create_task::create_task;
use tasks::delete_task::delete_task;
use tasks::get_all_tasks::get_all_tasks;
use tasks::get_task_by_id::get_task_by_id;
use tasks::partial_update_tasks::partial_update_tasks;

use users::create_user::create_user;
use users::guard::guard;
use users::login::login;
use users::logout::logout;

pub async fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::PUT])
        .allow_origin(Any);

    Router::new()
        .route("/users/logout", post(logout))
        .route_layer(middleware::from_fn(guard))
        .route("/tasks", post(create_task))
        .route("/tasks", get(get_all_tasks))
        .route("/tasks/:id", get(get_task_by_id))
        .route("/tasks/:id", put(atomic_update_tasks))
        .route("/tasks/:id", patch(partial_update_tasks))
        .route("/tasks/:id", delete(delete_task))
        .route("/users", post(create_user))
        .route("/users/login", post(login))
        .layer(Extension(database))
        .layer(cors)
}
