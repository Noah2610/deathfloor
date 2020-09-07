pub mod prelude {
    pub use super::actions::prelude as action;
    pub use super::event_type_data;
    pub use super::ActionTrigger;
    pub use super::ActionType;
    pub use super::ActionTypeTrigger;
    pub use super::EventType;
    pub use super::EventsRegister;
}

pub mod actions;
pub mod event_type_data;

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
