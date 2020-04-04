pub mod prelude {
    pub use super::animation_action::AnimationAction;
    pub use super::AnimationEditor;
}

mod animation_action;

use super::component_prelude::*;
use crate::animation_key::AnimationKey;
use animation_action::AnimationAction;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct AnimationEditor {
    actions: Vec<AnimationAction>,
}

impl AnimationEditor {
    pub fn add_action(&mut self, action: AnimationAction) {
        self.actions.push(action);
    }

    pub fn drain_actions(&mut self) -> std::vec::Drain<AnimationAction> {
        self.actions.drain(..)
    }
}
