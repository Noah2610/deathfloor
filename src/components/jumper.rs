use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
pub struct Jumper {
    pub strength:      f32,
    pub kill_strength: f32,
    pub gravity:       (Option<f32>, Option<f32>),
    pub min_velocity:  f32,
    #[serde(skip)]
    pub is_jumping:    bool,
}
