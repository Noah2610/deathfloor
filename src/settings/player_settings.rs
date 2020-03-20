use super::hitbox_config::HitboxConfig;
use super::prelude::AnimationsConfig;
use super::SizeSettings;
use crate::components::prelude::{Jumper, PhysicsData, WallJumper, WallSlider};

pub mod prelude {
    pub use super::PlayerSettings;
    pub use super::ShooterBulletData;
    pub use super::ShooterData;
}

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:        SizeSettings,
    pub physics:     PhysicsData,
    pub hitbox:      Option<HitboxConfig>,
    pub jumper:      Jumper,
    pub wall_jumper: Option<WallJumper>,
    pub wall_slider: Option<WallSlider>,
    pub shooter:     ShooterData,
    pub animations:  AnimationsConfig,
}

#[derive(Clone, Deserialize)]
pub struct ShooterData {
    pub cooldown_ms: u64,
    pub bullet:      ShooterBulletData,
}

#[derive(Clone, Deserialize)]
pub struct ShooterBulletData {
    pub size:             (f32, f32),
    pub velocity:         (f32, f32),
    pub despawn_after_ms: u64,
}
