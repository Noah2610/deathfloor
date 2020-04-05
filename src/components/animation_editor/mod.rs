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

impl ActionQueue for AnimationEditor {
    type Action = AnimationAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
