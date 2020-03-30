use super::component_prelude::*;
use super::ActionType;
use std::vec::Drain;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct ActionTypeTrigger {
    action_types: Vec<ActionType>,
}

impl ActionTypeTrigger {
    /// Trigger an `ActionType`-wrapped `Action`.
    pub fn trigger(&mut self, action_type: ActionType) {
        self.action_types.push(action_type);
    }

    /// Drain all triggered `ActionType`s.
    pub fn drain(&mut self) -> Drain<ActionType> {
        self.action_types.drain(..)
    }
}
