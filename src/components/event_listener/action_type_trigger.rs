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
        self.add_action(action_type);
    }

    /// Drain all triggered `ActionType`s.
    pub fn drain(&mut self) -> Drain<ActionType> {
        self.drain_actions()
    }
}

impl ActionQueue for ActionTypeTrigger {
    type Action = ActionType;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.action_types
    }
}
