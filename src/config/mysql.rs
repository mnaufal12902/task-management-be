use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

pub async fn establish_mysql_connection() -> MySqlPool {
    let database_url = env::var("MYSQL_URL").expect("MYSQL_URL must be set");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create MySQL pool")
}
