use super::component_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Component, Default, Clone, Deserialize)]
#[storage(DenseVecStorage)]
#[serde(from = "BulletData")]
pub struct Bullet {
    pub despawn_timer: Timer,
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

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BulletData {
    pub despawn_after_ms: u64,
}
