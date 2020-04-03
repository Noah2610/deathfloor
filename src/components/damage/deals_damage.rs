use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct DealsDamage {
    pub damage: HitPoints,
}
