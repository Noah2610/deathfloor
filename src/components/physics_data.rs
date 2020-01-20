use super::component_prelude::*;

#[derive(Clone, Component, Deserialize)]
#[storage(DenseVecStorage)]
pub struct PhysicsData {
    pub acceleration: (Option<f32>, Option<f32>),
    pub max_velocity: (Option<f32>, Option<f32>),
    pub damping:      f32,
    pub mass:         f32,
    pub has_gravity:  bool,
}
