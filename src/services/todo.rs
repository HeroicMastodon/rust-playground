use crate::guid::Guid;
use crate::routes::todo::{Status, Todo};

pub async fn list_tasks() -> Vec<Todo> {
    vec![
        Todo {
            id: Guid::new(),
            status: Status::Incomplete,
            name: "Task - 1".to_string(),
        },
        Todo {
            id: Guid::new(),
            status: Status::Incomplete,
            name: "Task - 2".to_string(),
        },
    ]
}

pub async fn get_task_by_id(id: Guid) -> Todo {
    Todo {
        id,
        name: format!("Task - {id}"),
        status: Status::Incomplete,
    }
}

pub async fn update_task(todo: Todo) -> Todo {
    Todo {
        ..todo
    }
}

pub async fn create_task(name: String) -> Todo {
    Todo {
        id: Guid::new(),
        name,
        status: Status::Incomplete
    }
}
