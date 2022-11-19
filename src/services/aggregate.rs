use std::fmt::{Debug, Display, Formatter};
use serde_derive::{Deserialize, Serialize};
use crate::guid::Guid;
use crate::routes::todo::Status;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum TodoEvent {
    Create { name: String, id: Guid },
    ChangeName { new_name: String, version: u32 },
    ChangeStatus { status: Status, version: u32 },
    Delete { version: u32 },
}

impl TodoEvent {
    pub(crate) fn version(&self) -> u32 {
        match self {
            TodoEvent::Create { .. } => { 0 }
            TodoEvent::ChangeName { version, .. } => { version.to_owned() }
            TodoEvent::ChangeStatus { version, .. } => { version.to_owned() }
            TodoEvent::Delete { version, .. } => { version.to_owned() }
        }
    }
}

pub enum AggregateErr {
    ConcurrencyErr,

}

impl Debug for AggregateErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateErr::ConcurrencyErr => {
                f.write_str("Attempted to apply multiple updates of the same version")
            }
        }
    }
}

impl Display for AggregateErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AggregateErr::ConcurrencyErr => {
                f.write_str("Attempted to apply multiple updates of the same version")
            }
        }
    }
}

#[derive(Clone)]
pub struct ValidTodoEvent {
    event: TodoEvent,
}

impl ValidTodoEvent {
    pub fn event(&self) -> TodoEvent {
        self.event.clone()
    }    
}

pub trait Aggregate {
    type Event;
    type ValidEvent;
    fn version(&self) -> u32;
    fn try_apply(&self, event: Self::Event) -> Result<Self::ValidEvent, AggregateErr>;
    fn apply(self, event: &Self::ValidEvent) -> Self;
    fn from_events(events: Vec<Self::Event>) -> Self;
}

#[derive(Debug)]
pub struct TodoAggregate {
    pub id: Guid,
    pub status: Status,
    pub name: String,
    version: u32,
    pub is_deleted: bool,
}

impl Aggregate for TodoAggregate {
    type Event = TodoEvent;
    type ValidEvent = ValidTodoEvent;

    fn version(&self) -> u32 {
        self.version
    }


    fn try_apply(&self, event: Self::Event) -> Result<ValidTodoEvent, AggregateErr> {
        if self.is_deleted {
            Err(AggregateErr::ConcurrencyErr)
        } else if event.version() != self.version + 1 {
            Err(AggregateErr::ConcurrencyErr)
        } else {
            Ok(ValidTodoEvent { event })
        }
    }

    fn apply(mut self, valid_event: &ValidTodoEvent) -> TodoAggregate {
        let event = &valid_event.event;
        self.version = event.version();
        match event {
            TodoEvent::Create { name, id } => {
                self.name = name.clone();
                self.id = id.clone();
            }
            TodoEvent::ChangeName { new_name, .. } => {
                self.name = new_name.clone();
            }
            TodoEvent::ChangeStatus { status, .. } => {
                self.status = status.clone();
            }
            TodoEvent::Delete { .. } => { self.is_deleted = true; }
        };

        self
    }

    fn from_events(events: Vec<Self::Event>) -> TodoAggregate {
        let mut agg = TodoAggregate {
            id: Guid::empty(),
            name: "".to_string(),
            is_deleted: false,
            version: 0,
            status: Status::Incomplete,
        };
        agg = events.into_iter().fold(agg, |agg, e| {
            let valid_event = agg.try_apply(e).unwrap();
            agg.apply(&valid_event)
        });

        agg
    }
}