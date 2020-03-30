pub mod prelude {
    pub use super::actions::prelude as action;
    pub use super::ActionTrigger;
    pub use super::ActionType;
    pub use super::ActionTypeTrigger;
    pub use super::EventType;
    pub use super::EventsRegister;
}

pub mod actions;

mod action_trigger;
mod action_type_trigger;
mod event_type;
mod events_register;

pub use action_trigger::ActionTrigger;
pub use action_type_trigger::ActionTypeTrigger;
pub use actions::ActionType;
pub use event_type::EventType;
pub use events_register::EventsRegister;

use super::component_prelude;

// TODO: cleanup

// use super::component_prelude::*;
// use std::collections::{HashMap, HashSet};

// #[derive(Component, Deserialize, Clone, Default)]
// #[storage(DenseVecStorage)]
// #[serde(from = "HashMap<EventType, Action>")]
// pub struct EventListener {
//     events:            HashMap<EventType, Action>,
//     event_types:       HashSet<EventType>,
//     triggered_actions: HashMap<ActionType, Vec<Action>>,
// }

// impl EventListener {
//     /// Returns all registered `EventType`s.
//     pub fn events(&self) -> &HashSet<EventType> {
//         &self.event_types
//     }

//     /// Triggers actions from the given event.
//     pub fn trigger(&mut self, event: &EventType) {
//         if let Some(action) = self.events.get(event).cloned() {
//             self.trigger_action(action);
//         }
//     }

//     /// Triggers an action directly.
//     pub fn trigger_action(&mut self, action: Action) {
//         self.triggered_actions
//             .entry((&action).into())
//             .or_insert_with(Default::default)
//             .push(action);
//     }

//     pub fn take_actions(
//         &mut self,
//         action_type: &ActionType,
//     ) -> Option<Vec<Action>> {
//         self.triggered_actions.remove(action_type)
//     }
// }

// impl From<HashMap<EventType, Action>> for EventListener {
//     fn from(events: HashMap<EventType, Action>) -> Self {
//         let event_types = events.keys().cloned().collect();
//         Self {
//             events,
//             event_types,
//             triggered_actions: Default::default(),
//         }
//     }
// }
