use super::component_prelude::*;
use crate::settings::prelude::ShooterBulletData;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Default, Clone, Deserialize)]
#[storage(DenseVecStorage)]
#[serde(from = "BulletData")]
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

impl From<BulletData> for Bullet {
    fn from(data: BulletData) -> Self {
        Self {
            despawn_timer: Timer::new(
                Some(Duration::from_millis(data.despawn_after_ms).into()),
                None,
            ),
        }
    }
}

#[derive(Deserialize)]
pub struct BulletData {
    pub despawn_after_ms: u64,
}
