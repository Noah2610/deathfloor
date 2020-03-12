use super::hitbox_config::HitboxConfig;
use super::SizeSettings;
use crate::components::prelude::{
    Jumper,
    MovementData,
    WallJumper,
    WallSlider,
};

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:        SizeSettings,
    pub movement:    MovementData,
    pub hitbox:      Option<HitboxConfig>,
    pub jumper:      Jumper,
    pub wall_jumper: Option<WallJumper>,
    pub wall_slider: Option<WallSlider>,
}
