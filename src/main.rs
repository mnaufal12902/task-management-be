use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header, web};
use config::{mongodb::establish_mongodb_connection, mysql::establish_mysql_connection};
use dotenv::dotenv;
use env_logger::Env;
use middleware::auth::AuthMiddleware;
use std::env;

mod config;
mod controllers;
mod middleware;
mod models;
mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::new().default_filter_or("debug"));

    dotenv().ok();

    let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid SERVER_PORT");

    let mysql_conn = establish_mysql_connection().await;
    // let mongodb_conn = establish_mongodb_connection()
    //     .await
    //     .expect("Failed to connect to MongoDB");

    let secret_key = env::var("SECRET_KEY").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000") // change with your domain 
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![header::AUTHORIZATION, header::CONTENT_TYPE])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(mysql_conn.clone()))
            // .app_data(web::Data::new(mongodb_conn.clone()))
            .configure(routes::auth::config)
            .configure(routes::session::config)
            .configure(routes::users::config)
            .service(
                web::scope("/api")
                    .wrap(AuthMiddleware::new(secret_key.clone()))
                    .configure(routes::tasks::config)
                    .configure(routes::course::config)
                    .configure(routes::user_tasks::config)
                    .configure(routes::group_tasks::config)
                    .configure(routes::group::config),
            )
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
