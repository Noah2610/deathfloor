// resources/settings/player.ron

use super::hitbox_config::HitboxConfig;
use super::SizeSettings;
use crate::animation_key::AnimationKey;
use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::*;
use crate::settings::entity_config::EntityConfig;
use deathframe::animation::components::prelude::AnimationsContainer;

pub mod prelude {
    pub use super::PlayerSettings;
    pub use super::ShooterData;
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
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
    pub takes_damage:   TakesDamage,

    #[serde(alias = "entity")]
    pub entity_config: Option<EntityConfig>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShooterData {
    pub cooldown_ms: u64,
}
