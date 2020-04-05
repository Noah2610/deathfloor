use super::component_prelude::*;
use super::sound_action::SoundAction;

#[derive(Component, Default)]
#[storage(VecStorage)]
pub struct SoundPlayer {
    actions: Vec<SoundAction>,
}

impl ActionQueue for SoundPlayer {
    type Action = SoundAction;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.actions
    }
}
