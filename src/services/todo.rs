use mongodb::{Client, Collection};
use mongodb::bson::doc;
use rocket::futures::{StreamExt};
use crate::guid::Guid;
use crate::routes::todo::{Status, Todo};


#[async_trait]
pub trait TodoService: Send + Sync {
    async fn list_tasks(&self) -> Result<Vec<Todo>, TodoServiceErr>;

    async fn get_task_by_id(&self, id: Guid) -> Result<Todo, TodoServiceErr>;

    async fn update_task(&self, todo: Todo) -> Result<Todo, TodoServiceErr>;

    async fn create_task(&self, name: String) -> Result<Todo, TodoServiceErr>;
}

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

pub struct MongoTodoService {
    todo_collection: Collection<Todo>,
}

impl MongoTodoService {
    pub async fn init(mongodb: &Client) -> MongoTodoService {
        let db = mongodb.database("rust-test");
        let collection = db.collection("todo");

        MongoTodoService {
            todo_collection: collection,
        }
    }
}

fn service_err<T>(_message: &str) -> fn(T) -> TodoServiceErr {
    |_| TodoServiceErr::new("Error occured".to_string())
}

#[async_trait]
impl TodoService for MongoTodoService {
    async fn list_tasks(&self) -> Result<Vec<Todo>, TodoServiceErr> {
        let collection = &self.todo_collection;
        let mut results = collection
            .find(None, None).await
            .map_err(service_err("Could not retrieve todos"))?;
        let mut todos: Vec<Todo> = vec![];
        while let Some(doc) = results.next().await {
            todos.push(doc.expect("could not deserialize todo"));
        }

        Ok(todos)
    }

    async fn get_task_by_id(&self, id: Guid) -> Result<Todo, TodoServiceErr> {
        let collection = &self.todo_collection;
        let query = doc! {
            "_id": id.to_string()
        };
        let results = collection
            .find_one(query, None).await
            .map_err(service_err("Could not find task"))?;

        match results {
            None => { Err(TodoServiceErr::new("Could not find task".to_string())) }
            Some(todo) => { Ok(todo) }
        }
    }

    async fn update_task(&self, todo: Todo) -> Result<Todo, TodoServiceErr> {
        println!("Updating task: {todo:?}");
        let collection = &self.todo_collection;
        let query = doc! {
            "_id": todo.id.to_string()
        };
        let results = collection
            .find_one_and_replace(query, &todo, None).await
            .map_err(|x| TodoServiceErr::new(format!("Error Updating todo: {:?}", x)))?;

        match results {
            None => { Err(TodoServiceErr::new(format!("Error Updating Todo two"))) }
            Some(_) => { Ok(todo) }
        }
    }

    async fn create_task(&self, name: String) -> Result<Todo, TodoServiceErr> {
        let todo = Todo {
            id: Guid::new(),
            name,
            status: Status::Incomplete,
        };

        let collection = &self.todo_collection;
        collection
            .insert_one(&todo, None).await
            .map_err(|x| TodoServiceErr::new(format!("Could not create todo: {:?}", x)))
            .map(|_| todo)
    }
}
