use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use rust_server::run;
mod data; 

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = dotenv!("DATABASE_URL");
    run(database_url).await 
}
