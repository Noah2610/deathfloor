pub mod prelude {
    pub use super::Action as EventAction;
    pub use super::ActionType as EventActionType;
    pub use super::EventListener;
    pub use super::EventType;
}

mod action;
mod event_type;

pub use action::Action;
pub use action::ActionType;
pub use event_type::EventType;

use super::component_prelude::*;
use std::collections::hash_map::{HashMap, Keys};

#[derive(Component, Deserialize, Clone, Default)]
#[storage(DenseVecStorage)]
#[serde(from = "HashMap<EventType, Action>")]
pub struct EventListener {
    events:  HashMap<EventType, Action>,
    actions: HashMap<ActionType, Vec<Action>>,
}

impl EventListener {
    pub fn events(&self) -> Keys<EventType, Action> {
        self.events.keys()
    }

    pub fn trigger(&mut self, event: &EventType) {
        if let Some(action) = self.events.get(event).cloned() {
            self.actions
                .entry((&action).into())
                .or_insert_with(Default::default)
                .push(action);
        }
    }

    pub fn take_actions(
        &mut self,
        action_type: &ActionType,
    ) -> Option<Vec<Action>> {
        self.actions.remove(action_type)
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
