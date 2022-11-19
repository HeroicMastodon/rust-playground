use std::cmp::Ordering;
use mongodb::{Client, Collection};
use mongodb::bson::doc;
use rocket::futures::StreamExt;
use serde_derive::{Deserialize, Serialize};
use crate::guid::Guid;
use crate::services::aggregate::{Aggregate, AggregateErr, TodoAggregate, TodoEvent, ValidTodoEvent};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoEventColl {
    #[serde(rename = "_id")]
    pub id: Guid,
    events: Vec<TodoEvent>,
}

impl TodoEventColl {
    pub fn version(mut self) -> u32 {
        match self.events
            .last() {
            None => {
                0
            }
            Some(event) => {
                event.version()
            }
        }
    }
    pub fn new(id: Guid) -> TodoEventColl {
        TodoEventColl {
            id,
            events: vec![],
        }
    }

    pub fn events(&self) -> Vec<TodoEvent> {
        self.events.to_vec()
    }

    pub fn add_event(mut self, valid_event: ValidTodoEvent) {
        self.events.push(valid_event.event());
    }

    pub fn to_agg(&self) -> TodoAggregate {
        TodoAggregate::from_events(self.events.clone())
    }
}

pub struct TodoEventCollRepo {
    collection: Collection<TodoEventColl>,
}

impl TodoEventCollRepo {
    pub fn new(mongodb: Client) -> TodoEventCollRepo {
        TodoEventCollRepo {
            collection: mongodb.database("rust-test").collection("todo-events")
        }
    }

    pub async fn insert(&self, coll: &TodoEventColl) -> Result<(), String> {
        match self.collection.insert_one(coll, None).await {
            Ok(_) => { Ok(()) }
            Err(_) => { Err(format!("Could not insert events")) }
        }
    }

    pub async fn get(&self, id: Guid) -> Result<TodoEventColl, String> {
        let query = doc! {
            "_id": id.to_string()
        };

        self.collection
            .find_one(query, None).await
            .map_err(|_| "Could not find events".to_string())
            .and_then(|result| match result {
                None => { Err("Could not find events".to_string()) }
                Some(events) => { Ok(events) }
            })
    }
    
    pub async fn update(&self, coll: &TodoEventColl) -> Result<() , String> {
        let query = doc! {
            "_id": coll.id.to_string()
        };

        match self.collection.find_one_and_replace(query, coll, None).await {
            Ok(_) => {Ok(())}
            Err(_) => {Err("Could not update collection".to_string())}
        }
    }
    
    pub async fn list(&self) -> Result<Vec<TodoEventColl>, String> {
        let mut cursor = self.collection
            .find(None, None).await
            .map_err(|_| "Could not list".to_string())?;
        let mut results: Vec<TodoEventColl> = vec![];
        
        while let Some(result) = cursor.next().await {
            let coll = result.map_err(|x| "Could not deserialize".to_string())?;
            results.push(coll);
        };
        
        Ok(results)
    }
}