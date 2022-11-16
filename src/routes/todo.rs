use mongodb::Client;
use rocket::serde::json::Json;
use rocket::{Build, Rocket, State};
use serde_derive::{Deserialize, Serialize};
use crate::guid::Guid;
use crate::services::data::MongoTodoRepository;
use crate::services::todo::{TodoService};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    #[serde(rename = "_id")]
    pub id: Guid,
    pub status: Status,
    pub name: String,
}

impl Todo {
    pub fn new(name: String) -> Todo {
        Todo {
            name,
            status: Status::Incomplete,
            id: Guid::new()
        }
    }
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
pub async fn list_tasks(service: &State<TodoService>) -> Json<Vec<Todo>> {
    Json(service.list_tasks().await.expect("err"))
}

#[get("/<id>")]
pub async fn get_task_by_id(id: Guid, service: &State<TodoService>) -> Json<Todo> {
    Json(service.get_task_by_id(id).await.expect("err"))
}

#[put("/", format = "json", data = "<todo>")]
pub async fn update_task(todo: Json<Todo>, service: &State<TodoService>) -> Json<Todo> {
    Json(service.update_task(todo.into_inner()).await.expect("err"))
}

#[post("/", format = "json", data = "<name>")]
pub async fn create_task(name: Json<CreateTodoRequest>, service: &State<TodoService>) -> Json<Todo> {
    Json(service.create_task(name.into_inner().name).await.expect("err"))
}

#[async_trait]
pub trait AddTodo {
    async fn add_todo(self, mongodb: &Client) -> Rocket<Build>;
}

#[async_trait]
impl AddTodo for Rocket<Build> {
    async fn add_todo(self, mongodb: &Client) -> Rocket<Build> {
        let todo_repo = MongoTodoRepository::new(mongodb);
        let todo_service = TodoService::init(Box::new(todo_repo)).await;
        
        self
            .manage(todo_service)
            .mount("/api/todo", routes![
                create_task,
                update_task,
                list_tasks,
                get_task_by_id
            ])
    }
}
