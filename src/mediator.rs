use mongodb::Client;
use crate::services::create_mongo_client;
use crate::services::todo::{MongoTodoService, TodoService};

pub struct Services {
    pub todo_service: Box<dyn TodoService + Send + Sync>,
}

impl Services {
    pub async fn init() -> Services {
        let client = &create_mongo_client().await;
        Services {
            todo_service: Box::new(MongoTodoService::init(client).await)
        }
    }
}

