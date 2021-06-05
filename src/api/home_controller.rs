use actix_web::{HttpResponse, Result, web, Error};
use serde::{Serialize, Deserialize};
use crate::model::Status;
use deadpool_postgres::{Pool, Client};
use crate::db;
use crate::errors::MyError;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

pub async fn hello() -> HttpResponse {
    return HttpResponse::Ok().body("banh chung xanh xanh");
}

pub async fn echo() -> Result<HttpResponse> {
    return Ok(HttpResponse::Ok().json(MyObj{
        name: "minatokuda".to_string(),
    }));
}

pub async fn status() -> HttpResponse {
    return web::HttpResponse::Ok()
        .json(Status{status: "UP".to_string()})
}

pub async fn todo_list(db_pool: web::Data<Pool>) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let result = db::get_todos(&client).await;
    
    return result.map(|todos| HttpResponse::Ok().json(todos));
}