use deadpool_postgres::Client;
use crate::model::TodoList;
use crate::errors::MyError;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, MyError> {
    let statement = client.prepare("select * from todo_list").await.unwrap();
    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    
    return Ok(todos);
}