use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Jumper {
    pub is_jumping: bool,
}
