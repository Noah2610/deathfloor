use super::component_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(from = "ShooterDeser")]
pub struct Shooter {
    pub cooldown_timer: Timer,
}

impl From<ShooterDeser> for Shooter {
    fn from(data: ShooterDeser) -> Self {
        Self {
            cooldown_timer: Timer::new(
                Some(Duration::from_millis(data.cooldown_ms).into()),
                None,
            ),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShooterDeser {
    pub cooldown_ms: u64,
}
