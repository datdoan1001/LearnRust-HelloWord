mod api;
mod app_config;
mod model;
mod db;
mod errors;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use api::home_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok().expect("Failed to read .env file");
    
    let config = crate::app_config::Configuration::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at {}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(home_controller::hello))
            .route("/echo", web::get().to(home_controller::echo))
            .route("/status", web::get().to(home_controller::status))
            .route("/todo", web::get().to(home_controller::todo_list))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}