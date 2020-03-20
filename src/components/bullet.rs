use super::component_prelude::*;
use crate::settings::prelude::ShooterBulletData;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Bullet {
    pub despawn_timer: Timer,
}

impl From<&ShooterBulletData> for Bullet {
    fn from(data: &ShooterBulletData) -> Self {
        Self {
            despawn_timer: Timer::new(
                Some(Duration::from_millis(data.despawn_after_ms).into()),
                None,
            ),
        }
    }
}
