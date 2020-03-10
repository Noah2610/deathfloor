use super::component_prelude::*;
use crate::settings::prelude::ShootingData;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Builder)]
#[storage(VecStorage)]
#[builder(pattern = "owned")]
pub struct Shooter {
    pub cooldown_timer: Timer,
}

impl Shooter {
    pub fn builder() -> ShooterBuilder {
        ShooterBuilder::default()
    }
}

impl From<&ShootingData> for Shooter {
    fn from(data: &ShootingData) -> Self {
        Self {
            cooldown_timer: Timer::new(
                Some(Duration::from_millis(data.cooldown_ms).into()),
                None,
            ),
        }
    }
}
