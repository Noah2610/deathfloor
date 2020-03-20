use super::component_prelude::*;
use crate::settings::prelude::{ShooterBulletData, ShooterData};
use climer::Timer;
use std::time::Duration;

#[derive(Component, Builder)]
#[storage(VecStorage)]
#[builder(pattern = "owned")]
pub struct Shooter {
    pub cooldown_timer: Timer,
    pub bullet_data:    ShooterBulletData,
}

impl Shooter {
    pub fn builder() -> ShooterBuilder {
        ShooterBuilder::default()
    }
}

impl From<ShooterData> for Shooter {
    fn from(data: ShooterData) -> Self {
        Self {
            cooldown_timer: Timer::new(
                Some(Duration::from_millis(data.cooldown_ms).into()),
                None,
            ),
            bullet_data:    data.bullet,
        }
    }
}