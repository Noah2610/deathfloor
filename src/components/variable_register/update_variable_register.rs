use super::super::component_prelude::*;
use crate::expression::ExpressionValue;

/// Component for storing variable update actions.
/// Used with `VariableAction`s as an intermediate step
/// between running action and actually setting variable value.
#[derive(Component, Clone, Default)]
pub struct UpdateVariableRegister {
    actions: Vec<UpdateVariableAction>,
}

#[derive(Clone)]
pub enum UpdateVariableAction {
    Set(String, ExpressionValue),
}

impl ActionQueue for UpdateVariableRegister {
    type Action = UpdateVariableAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
