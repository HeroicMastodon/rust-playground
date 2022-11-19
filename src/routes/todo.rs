use mongodb::Client;
use rocket::serde::json::Json;
use rocket::{Build, Request, Rocket, State};
use rocket::http::{ContentType, Header};
use serde_derive::{Deserialize, Serialize};
use crate::guid::Guid;
use crate::services::aggregate::{Aggregate, TodoAggregate, TodoEvent};
use crate::services::data::MongoTodoRepository;
use crate::services::event_store::TodoEventCollRepo;
use crate::services::todo::{TodoService, TodoServiceErr};

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: Guid,
    pub status: Status,
    pub name: String,
    pub version: u32,
}

impl Todo {
    pub fn new(name: String) -> Todo {
        Todo {
            name,
            status: Status::Incomplete,
            id: Guid::new(),
            version: 0,
        }
    }

    pub fn from_agg(agg: TodoAggregate) -> Todo {
        Todo {
            version: agg.version(),
            id: agg.id,
            status: agg.status,
            name: agg.name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Status {
    Complete,
    Incomplete,
}

#[derive(Deserialize, Serialize)]
struct TodoError {
    message: String,
}

impl TodoError {
    fn new(message: &str) -> TodoError {
        TodoError {
            message: message.to_string()
        }
    }
}

#[derive(Responder)]
#[response(status = 400, content_type = "json")]
pub struct TodoErrResponder {
    inner: Json<TodoServiceErr>,
}

impl TodoErrResponder {
    pub fn new(err: TodoServiceErr) -> TodoErrResponder {
        TodoErrResponder {
            inner: Json(err)
        }
    }
}

pub type ActionResult<T> = Result<Json<T>, TodoErrResponder>;

#[catch(422)]
async fn catch_malformed_request(req: &Request<'_>) -> Json<TodoError> {
    Json(TodoError::new("Could not parse request"))
}

#[get("/")]
pub async fn list_tasks(service: &State<TodoService>) -> ActionResult<Vec<Todo>> {
    let todos = service
        .list_tasks().await
        .map_err(TodoErrResponder::new)?
        .into_iter()
        .map(|agg| Todo::from_agg(agg))
        .collect();

    Ok(Json(todos))
}

#[get("/<id>")]
pub async fn get_task_by_id(id: Guid, service: &State<TodoService>) -> ActionResult<Todo> {
    let agg = service.get_task_by_id(id).await.map_err(TodoErrResponder::new)?;
    let todo = Todo::from_agg(agg);

    Ok(Json(todo))
}

#[put("/<id>", format = "json", data = "<event>")]
pub async fn update_task(id: Guid, event: Json<TodoEvent>, service: &State<TodoService>) -> ActionResult<Todo> {
    let agg = service.update_task(id, event.into_inner()).await.map_err(TodoErrResponder::new)?;
    let todo = Todo::from_agg(agg);
    
    Ok(Json(todo))
}

#[post("/", format = "json", data = "<name>")]
pub async fn create_task(name: Json<CreateTodoRequest>, service: &State<TodoService>) -> ActionResult<Todo> {
    let agg = service.create_task(name.into_inner().name).await.map_err(TodoErrResponder::new)?;
    let todo = Todo::from_agg(agg);

    Ok(Json(todo))
}

#[async_trait]
pub trait AddTodo {
    async fn add_todo(self, mongodb: &Client) -> Rocket<Build>;
}

#[async_trait]
impl AddTodo for Rocket<Build> {
    async fn add_todo(self, mongodb: &Client) -> Rocket<Build> {
        let todo_repo = MongoTodoRepository::new(mongodb);
        let event_repo = TodoEventCollRepo::new(mongodb);
        let todo_service = TodoService::init(
            Box::new(todo_repo),
            event_repo,
        ).await;

        self
            .manage(todo_service)
            .mount("/api/todo", routes![
                create_task,
                update_task,
                list_tasks,
                get_task_by_id
            ])
            .register("/api/todo", catchers![
                catch_malformed_request
            ])
    }
}
