use super::component_prelude::*;

pub enum InteractableAction {
    Interacted,
}

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct Interactable {
    actions: Vec<InteractableAction>,
}

impl ActionQueue for Interactable {
    type Action = InteractableAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
