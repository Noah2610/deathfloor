use super::component_prelude::*;
use super::prelude::Gravity;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Jumper {
    pub strength:         f32,
    pub kill_strength:    f32,
    pub gravity:          Gravity,
    pub min_velocity:     f32,
    #[serde(skip)]
    pub is_jumping:       bool,
    #[serde(skip)]
    pub original_gravity: Option<Gravity>,
}
