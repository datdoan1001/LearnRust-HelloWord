use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}