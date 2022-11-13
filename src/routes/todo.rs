use rocket::serde::json::Json;
use rocket::State;
use serde_derive::{Deserialize, Serialize};
use crate::guid::Guid;
use crate::{Services};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    #[serde(rename = "_id")]
    pub id: Guid,
    pub status: Status,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Status {
    Complete,
    Incomplete,
}

#[get("/")]
pub async fn list_tasks(services: &State<Services>) -> Json<Vec<Todo>> {
    Json(services.todo_service.list_tasks().await.expect("err"))
}

#[get("/<id>")]
pub async fn get_task_by_id(id: Guid, services: &State<Services>) -> Json<Todo> {
    Json(services.todo_service.get_task_by_id(id).await.expect("err"))
}

#[put("/", format = "json", data = "<todo>")]
pub async fn update_task(todo: Json<Todo>, services: &State<Services>) -> Json<Todo> {
    Json(services.todo_service.update_task(todo.into_inner()).await.expect("err"))
}

#[post("/", format = "json", data = "<name>")]
pub async fn create_task(name: Json<CreateTodoRequest>, services: &State<Services>) -> Json<Todo> {
    Json(services.todo_service.create_task(name.into_inner().name).await.expect("err"))
}
