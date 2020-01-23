use super::component_prelude::*;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Grounded {
    on_ground: bool,
}

impl Grounded {
    pub fn is_on_ground(&self) -> bool {
        self.on_ground
    }

    pub fn is_in_air(&self) -> bool {
        !self.on_ground
    }
}
