use super::hitbox_config::HitboxConfig;
use super::SizeSettings;
use crate::components::prelude::MovementData;

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:     SizeSettings,
    pub movement: MovementData,
    pub hitbox:   Option<HitboxConfig>,
}
