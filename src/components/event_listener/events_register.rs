use super::component_prelude::*;
use super::{ActionType, EventType};
use crate::merge::Merge;
use std::collections::HashMap;

#[derive(Component, Deserialize, Clone)]
#[storage(DenseVecStorage)]
#[serde(from = "HashMap<EventType, ActionType>")]
pub struct EventsRegister {
    events: HashMap<EventType, ActionType>,
}

impl EventsRegister {
    /// Returns all registered `EventType`s.
    pub fn events(&self) -> &HashMap<EventType, ActionType> {
        &self.events
    }

    /// Returns the `ActionType` for the given `EventType`.
    pub fn get_action(&self, event: &EventType) -> Option<&ActionType> {
        self.events.get(event)
    }
}

impl From<HashMap<EventType, ActionType>> for EventsRegister {
    fn from(events: HashMap<EventType, ActionType>) -> Self {
        Self { events }
    }
}

impl Merge for EventsRegister {
    fn merge(&mut self, other: Self) {
        self.events.extend(other.events);
    }
}
