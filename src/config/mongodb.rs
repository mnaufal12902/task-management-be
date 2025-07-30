use mongodb::{Client, options::ClientOptions};
use std::env;
use mongodb::Database;
use mongodb::error::Result;

pub async fn establish_mongodb_connection() -> Result<Database> {
    let database_url = env::var("MONGODB_URL")
        .expect("MONGODB_URL must be set in .env");
    let client_options = ClientOptions::parse(&database_url)
        .await
        .expect("Failed to parse MongoDB URL");
    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");
    let db = client.database("task_management");  

    Ok(db)
}
