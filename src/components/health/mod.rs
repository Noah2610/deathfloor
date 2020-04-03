pub mod prelude {
    pub use super::health_action::HealthAction;
    pub use super::health_display::{HealthDisplay, HealthDisplayPosition};
    pub use super::health_editor::HealthEditor;
    pub use super::Health;
    pub use super::HitPoints;
}

mod health_action;
mod health_display;
mod health_editor;

use super::component_prelude::{self, *};

pub type HitPoints = u32;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct Health {
    pub health:     HitPoints,
    pub max_health: HitPoints,
}

impl Health {
    pub fn gain(&mut self, hp: HitPoints) {
        self.health = (self.health + hp).min(self.max_health);
    }

    pub fn lose(&mut self, hp: HitPoints) {
        self.health = self.health.checked_sub(hp).unwrap_or(0);
    }
}
