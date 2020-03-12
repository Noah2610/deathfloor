use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct WallSlider {
    pub slide_velocity: f32,
}
