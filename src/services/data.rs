use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use rocket::futures::StreamExt;
use crate::guid::Guid;
use crate::routes::todo::Todo;

pub type DataAccessResult<T> = Result<T, DataAccessErr>;

#[derive(Debug)]
pub struct DataAccessErr {
    pub message: String,
}

impl DataAccessErr {
    fn new(message: &str) -> DataAccessErr {
        DataAccessErr {
            message: message.to_string()
        }
    }
}

impl Display for DataAccessErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for DataAccessErr {}

#[async_trait]
pub trait TodoRepository: Send + Sync {
    async fn insert(&self, todo: Todo) -> DataAccessResult<Todo>;
    async fn update(&self, todo: Todo) -> DataAccessResult<Todo>;
    async fn get_by_id(&self, id: Guid) -> DataAccessResult<Todo>;
    async fn list(&self) -> DataAccessResult<Vec<Todo>>;
}

pub struct MongoTodoRepository {
    collection: Collection<Todo>,
}

impl MongoTodoRepository {
    pub fn new(client: &Client) -> MongoTodoRepository {
        MongoTodoRepository {
            collection: client.database("rust-test").collection("todo")
        }
    }
}

#[async_trait]
impl TodoRepository for MongoTodoRepository {
    async fn insert(&self, todo: Todo) -> DataAccessResult<Todo> {
        self.collection
            .insert_one(&todo, None).await
            .map_err(|_| DataAccessErr::new("Could not Insert todo {todo:?}"))?;

        Ok(todo)
    }

    async fn update(&self, todo: Todo) -> DataAccessResult<Todo> {
        let query = doc! {
            "_id": todo.id.to_string()
        };
        self.collection
            .find_one_and_replace(query, &todo, None).await
            .map_err(|_| DataAccessErr::new("Could not update todo {todo:?}"))?;

        Ok(todo)
    }

    async fn get_by_id(&self, id: Guid) -> DataAccessResult<Todo> {
        let query = doc! {
          "_id": id.to_string()  
        };
        match self.collection
            .find_one(query, None).await
            .map_err(|_| DataAccessErr::new("Could not find todo"))? {
            None => { Err(DataAccessErr::new("Could not find todo")) }
            Some(todo) => { Ok(todo) }
        }
    }

    async fn list(&self) -> DataAccessResult<Vec<Todo>> {
        let mut cursor = self.collection
            .find(None, None).await
            .map_err(|_| DataAccessErr::new("Could not list todos"))?;
        let mut results: Vec<Todo> = vec![];

        while let Some(todo) = cursor.next().await {
            results.push(todo.map_err(|_| DataAccessErr::new("Could not deserialize todo"))?);
        };

        Ok(results)
    }
}