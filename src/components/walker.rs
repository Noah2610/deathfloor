use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
pub struct Walker {
    pub x: Option<f32>,
    pub y: Option<f32>,
}
