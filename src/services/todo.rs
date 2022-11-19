use std::error::Error;
use std::fmt::{Display, Formatter};
use std::string::ToString;
use serde_derive::Serialize;
use crate::guid::Guid;
use crate::routes::todo::{Todo};
use crate::services::aggregate::{Aggregate, AggregateErr, TodoAggregate, TodoEvent};
use crate::services::data::{DataAccessErr, TodoRepository};
use crate::services::event_store::{TodoEventColl, TodoEventCollRepo};

#[derive(Debug, Serialize)]
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
    event_repo: TodoEventCollRepo,
}

// const MAP_DATA_ERR: fn(DataAccessErr) -> TodoServiceErr = |x: DataAccessErr| TodoServiceErr::new(x.message);
const MAP_STRING_ERR: fn(String) -> TodoServiceErr = |message: String| TodoServiceErr { message };
const MAP_AGG_ERR: fn(AggregateErr) -> TodoServiceErr = |x: AggregateErr| TodoServiceErr {message: x.to_string()};

impl TodoService {
    pub async fn init(todo_repo: Box<dyn TodoRepository>, event_repo: TodoEventCollRepo) -> TodoService {
        TodoService {
            todo_repo,
            event_repo,
        }
    }

    pub async fn list_tasks(&self) -> Result<Vec<TodoAggregate>, TodoServiceErr> {
        self.event_repo
            .list().await
            .map_err(MAP_STRING_ERR)
            .map(|events| {
                events
                    .into_iter()
                    .map(|event| event.to_agg())
                    .collect()
            })
    }

    pub async fn get_task_by_id(&self, id: Guid) -> Result<TodoAggregate, TodoServiceErr> {
        self.event_repo
            .get(id).await
            .map_err(MAP_STRING_ERR)
            .map(|events| events.to_agg())
    }

    pub async fn update_task(&self, id:Guid, event: TodoEvent) -> Result<TodoAggregate, TodoServiceErr> {
        let mut coll = self.event_repo
            .get(id).await
            .map_err(MAP_STRING_ERR)?;
        let mut agg = coll.to_agg();
        
        let valid_event = agg
            .try_apply(event)
            .map_err(MAP_AGG_ERR)?;
        agg = agg.apply(&valid_event);
        coll = coll.add_event(valid_event);
        
        self.event_repo
            .update(&coll).await
            .map_err(MAP_STRING_ERR)?;
        
        Ok(agg)
    }

    pub async fn create_task(&self, name: String) -> Result<TodoAggregate, TodoServiceErr> {
        let id = Guid::new();
        let event = TodoEvent::Create { name, id: id.clone() };
        let mut coll = TodoEventColl::new(id.clone());
        let mut agg = TodoAggregate::new();
        
        let valid_event = agg.try_apply(event).map_err(MAP_AGG_ERR)?;
        agg = agg.apply(&valid_event);
        coll = coll.add_event(valid_event);
        
        self.event_repo
            .insert(&coll).await
            .map_err(MAP_STRING_ERR)?;
        
        Ok(agg)
    }
}
