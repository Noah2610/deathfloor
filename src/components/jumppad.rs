use super::component_prelude::*;

#[derive(Component, Deserialize, Clone, Default)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Jumppad {
    pub strength: (Option<f32>, Option<f32>),
}
