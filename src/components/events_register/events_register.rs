use super::component_prelude::*;
use super::{ActionType, EventType};
use std::collections::HashMap;

#[derive(Default)]
pub struct EventsRegisterData {
    pub interval: HashMap<u64, event_type_data::IntervalData>,
    pub delay:    HashMap<u64, event_type_data::DelayData>,
    pub init:     event_type_data::InitData,
}

impl Clone for EventsRegisterData {
    fn clone(&self) -> Self {
        Self::default()
    }
}

#[derive(Component, Deserialize, Clone)]
#[storage(DenseVecStorage)]
#[serde(from = "HashMap<EventType, ActionType>")]
#[serde(deny_unknown_fields)]
pub struct EventsRegister {
    events:   HashMap<EventType, ActionType>,
    pub data: EventsRegisterData,
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
        Self {
            events,
            data: Default::default(),
        }
    }
}

impl Merge for EventsRegister {
    fn merge(&mut self, other: Self) {
        for (other_event_type, other_action_type) in other.events {
            if let Some(existing_action_type) =
                self.events.remove(&other_event_type)
            {
                self.events.insert(
                    other_event_type,
                    ActionType::Group(
                        action::Group(vec![
                            existing_action_type,
                            other_action_type,
                        ])
                        .flattend(),
                    ),
                );
            } else {
                self.events.insert(other_event_type, other_action_type);
            }
        }
    }
}
