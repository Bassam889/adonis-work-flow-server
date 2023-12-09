use std::net::SocketAddr;

// use app_state::AppState;
// use router::create_router;

// pub mod app_state;
mod database;
mod utils;
mod routes;

use routes::create_routes;

use sea_orm::Database;
// pub async fn run(app_state: AppState) {
//     let app = create_router(app_state);
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("Listening on port {addr}...\n");
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
pub async fn run(database_url: &str) {
    match Database::connect(database_url).await {
        Ok(database) => {
            let app = create_routes(database).await;
            let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
            println!("Listening on port {addr}...\n");
            if let Err(err) = axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
            {
                println!("Error starting server: {:?}", err);
            }
        }
        Err(err) => {
            println!("Error connecting to the database: {:?}", err);
        }
    }
}
