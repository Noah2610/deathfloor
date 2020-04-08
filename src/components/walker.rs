use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Walker {
    pub x: Option<f32>,
    pub y: Option<f32>,
}
