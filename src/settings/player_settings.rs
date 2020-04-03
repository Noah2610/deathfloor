use super::hitbox_config::HitboxConfig;
use super::SizeSettings;
use crate::animation_key::AnimationKey;
use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::*;
use deathframe::animation::components::prelude::{
    Animation,
    AnimationsContainer,
};
use deathframe::animation::data::prelude::AnimationTypeWrapper;

pub mod prelude {
    pub use super::PlayerSettings;
    pub use super::ShooterBulletData;
    pub use super::ShooterData;
}

#[derive(Clone, Deserialize)]
pub struct PlayerSettings {
    pub size:           SizeSettings,
    pub physics:        PhysicsData,
    pub hitbox:         HitboxConfig,
    pub collision_tag:  CollisionTagWrapper,
    pub solid_tag:      CollisionTagWrapper,
    pub jumper:         Jumper,
    pub wall_jumper:    Option<WallJumper>,
    pub wall_slider:    Option<WallSlider>,
    pub shooter:        ShooterData,
    pub animations:     AnimationsContainer<AnimationKey>,
    pub health:         Health,
    pub health_display: HealthDisplay,
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
    pub animation:        AnimationTypeWrapper<Animation>,
}
