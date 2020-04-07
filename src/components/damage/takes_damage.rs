use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct TakesDamage {
    pub takes_damage_from: Vec<CollisionLabel>,
}
