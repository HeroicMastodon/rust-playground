use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::guid::Guid;
use crate::routes::todo::{Todo};
use crate::services::data::{DataAccessErr, TodoRepository};

#[derive(Debug)]
pub struct TodoServiceErr {
    message: String,
}

impl TodoServiceErr {
    pub fn new(message: String) -> TodoServiceErr {
        TodoServiceErr {
            message
        }
    }
}

impl Display for TodoServiceErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for TodoServiceErr {}

pub struct TodoService {
    todo_repo: Box<dyn TodoRepository>,
}

const MAP_DATA_ERR: fn(DataAccessErr) -> TodoServiceErr = |x: DataAccessErr| TodoServiceErr::new(x.message);

impl TodoService {
    pub async fn init(todo_repo: Box<dyn TodoRepository>) -> TodoService {
        TodoService {
            todo_repo
        }
    }

    pub async fn list_tasks(&self) -> Result<Vec<Todo>, TodoServiceErr> {
        self.todo_repo
            .list().await
            .map_err(MAP_DATA_ERR)
    }

    pub async fn get_task_by_id(&self, id: Guid) -> Result<Todo, TodoServiceErr> {
        self.todo_repo
            .get_by_id(id).await
            .map_err(MAP_DATA_ERR)
    }

    pub async fn update_task(&self, todo: Todo) -> Result<Todo, TodoServiceErr> {
        self.todo_repo
            .update(todo).await
            .map_err(MAP_DATA_ERR)
    }

    pub async fn create_task(&self, name: String) -> Result<Todo, TodoServiceErr> {
        self.todo_repo
            .insert(Todo::new(name)).await
            .map_err(MAP_DATA_ERR)
    }
}
