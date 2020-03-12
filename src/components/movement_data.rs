use super::component_prelude::*;

#[derive(Clone, Component, Deserialize)]
#[storage(DenseVecStorage)]
pub struct MovementData {
    pub acceleration:        (Option<f32>, Option<f32>),
    pub max_velocity:        (Option<f32>, Option<f32>),
    pub base_friction:       (Option<f32>, Option<f32>),
    pub gravity:             (Option<f32>, Option<f32>),
    pub wall_jump_strength:  (f32, f32),
    pub wall_slide_strength: f32,
}
