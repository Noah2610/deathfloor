pub mod prelude {
    pub use super::Action as EventAction;
    pub use super::EventListener;
    pub use super::EventType;
}

mod action;
mod event_type;

pub use action::Action;
pub use event_type::EventType;

use super::component_prelude::*;
use std::collections::HashMap;

#[derive(Component, Deserialize, Clone, Default)]
#[storage(DenseVecStorage)]
#[serde(from = "HashMap<EventType, Action>")]
pub struct EventListener {
    pub events:  HashMap<EventType, Action>,
    pub actions: Vec<Action>,
}

impl EventListener {
    pub fn trigger(&mut self, event: &EventType) {
        if let Some(action) = self.events.get(event).cloned() {
            self.actions.push(action);
        }
    }
}

impl From<HashMap<EventType, Action>> for EventListener {
    fn from(events: HashMap<EventType, Action>) -> Self {
        Self {
            events,
            actions: Default::default(),
        }
    }
}
