use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Jumppad {
    pub strength: (f32, f32),
}
