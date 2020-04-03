pub mod prelude {
    pub use super::health_display::{
        HealthDisplay,
        HealthDisplayMarker,
        HealthDisplayPosition,
    };
    pub use super::Health;
    pub use super::HitPoints;
}

mod health_display;

use super::component_prelude::{self, *};

pub type HitPoints = u32;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Health {
    pub health:     HitPoints,
    pub max_health: HitPoints,
}
