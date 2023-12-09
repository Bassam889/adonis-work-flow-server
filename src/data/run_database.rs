use sea_orm::Database;

pub async fn run_database(database_url: &str) {
    let _database = Database::connect(database_url).await;
}
