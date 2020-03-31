pub mod prelude {
    pub use super::CollisionTagData;
    pub use super::EnemiesSettings;
    pub use super::EnemyComponentsData;
    pub use super::EnemySettings;
}

use super::hitbox_config::HitboxConfig;
use crate::animation_key::AnimationKey;
use crate::collision_tag::EnemyCollidesWith;
use crate::components::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct EnemiesSettings {
    pub types: HashMap<EnemyType, EnemySettings>,
}

#[derive(Clone, Deserialize)]
pub struct EnemySettings {
    pub spritesheet_filename: String, // TODO
    pub components:           Option<EnemyComponentsData>,
    pub events:               Option<EventsRegister>,
    pub collision_tag:        Option<CollisionTagData>,
    pub solid_tag:            Option<CollisionTagData>,
}

/// List of posible components for an Enemy.
/// All can optionally be added to an enemy's config.
#[derive(Clone, Deserialize)]
pub struct EnemyComponentsData {
    pub size:                  Option<Size>,
    pub gravity:               Option<Gravity>,
    pub max_movement_velocity: Option<MaxMovementVelocity>,
    pub base_friction:         Option<BaseFriction>,
    pub animations:            Option<AnimationsContainer<AnimationKey>>,
    pub hitbox:                Option<HitboxConfig>,
    pub walker:                Option<Walker>,
    pub jumppad:               Option<Jumppad>,
}

#[derive(Clone, Deserialize)]
pub struct CollisionTagData {
    pub labels:        Vec<CollisionLabel>,
    pub collides_with: Option<Vec<CollisionLabel>>,
}
