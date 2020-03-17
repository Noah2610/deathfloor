use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Jumppad {
    pub strength: (Option<f32>, Option<f32>),
}
