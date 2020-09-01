use super::component_prelude::*;
use super::ActionType;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct ActionTypeTrigger {
    action_types: Vec<ActionType>,
}

impl ActionQueue for ActionTypeTrigger {
    type Action = ActionType;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.action_types
    }
}
