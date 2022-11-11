use rocket::serde::json::Json;
use serde_derive::{Deserialize, Serialize};
use crate::guid::Guid;
use crate::services;

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub status: Status,
    pub name: String,
    pub id: Guid,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Complete,
    Incomplete,
}

#[get("/")]
pub async fn list_tasks() -> Json<Vec<Todo>> {
    Json(services::todo::list_tasks().await)
}

#[get("/<id>")]
pub async fn get_task_by_id(id: Guid) -> Json<Todo> {
    Json(services::todo::get_task_by_id(id).await)
}

#[put("/", format = "json", data = "<todo>")]
pub async fn update_task(todo: Json<Todo>) -> Json<Todo> {
    Json(services::todo::update_task(todo.into_inner()).await)
}

#[post("/", format = "json", data = "<name>")]
pub async fn create_task(name: Json<CreateTodoRequest>) -> Json<Todo> {
    Json(services::todo::create_task(name.into_inner().name).await)
}
