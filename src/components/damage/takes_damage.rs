use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct TakesDamage {
    pub takes_damage_from: Vec<CollisionLabel>,
}
