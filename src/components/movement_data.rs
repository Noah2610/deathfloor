use super::component_prelude::*;

#[derive(Clone, Component, Deserialize)]
#[storage(DenseVecStorage)]
pub struct MovementData {
    pub acceleration:       (Option<f32>, Option<f32>),
    pub max_velocity:       (Option<f32>, Option<f32>),
    pub base_friction:      (Option<f32>, Option<f32>),
    pub gravity:            (Option<f32>, Option<f32>),
    pub jump_strength:      f32,
    pub jump_kill_strength: f32,
    pub min_jump_velocity:  f32,
}
