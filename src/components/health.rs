pub mod prelude {
    pub use super::Health;
    pub use super::HitPoints;
}

use super::component_prelude::*;

pub type HitPoints = u32;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Health {
    pub health:     HitPoints,
    pub max_health: HitPoints,
}
