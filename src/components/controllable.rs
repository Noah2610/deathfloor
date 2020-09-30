use super::component_prelude::*;

#[derive(Component, Deserialize, Clone, Default)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct Controllable {
    pub is_controllable: bool,
}
