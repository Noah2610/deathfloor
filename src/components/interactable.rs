use super::component_prelude::*;

#[derive(Clone)]
pub enum InteractableAction {
    Interacted,
}

#[derive(Component, Default, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Interactable {
    #[serde(skip)]
    actions: Vec<InteractableAction>,
}

impl ActionQueue for Interactable {
    type Action = InteractableAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
