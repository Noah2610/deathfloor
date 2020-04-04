pub mod prelude {
    pub use super::EnemiesSettings;
    pub use super::EnemyComponentsData;
    pub use super::EnemyComponentsStorages;
    pub use super::EnemySettings;
}

use super::hitbox_config::HitboxConfig;
use crate::animation_key::AnimationKey;
use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::*;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{SystemData, World, WriteStorage};
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
    pub collision_tag:        Option<CollisionTagWrapper>,
    pub solid_tag:            Option<CollisionTagWrapper>,
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
    pub scale_once:            Option<ScaleOnce>,
    pub health:                Option<Health>,
    pub health_display:        Option<HealthDisplay>,
    pub deals_damage:          Option<DealsDamage>,
    pub takes_damage:          Option<TakesDamage>,
}

#[derive(SystemData)]
pub struct EnemyComponentsStorages<'a> {
    pub size:                  WriteStorage<'a, Size>,
    pub gravity:               WriteStorage<'a, Gravity>,
    pub max_movement_velocity: WriteStorage<'a, MaxMovementVelocity>,
    pub base_friction:         WriteStorage<'a, BaseFriction>,
    pub animations: WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    pub animation_editor:      WriteStorage<'a, AnimationEditor>,
    pub hitbox:                WriteStorage<'a, Hitbox>,
    pub walker:                WriteStorage<'a, Walker>,
    pub jumppad:               WriteStorage<'a, Jumppad>,
    pub jumppad_affected:      WriteStorage<'a, JumppadAffected>,
    pub scale_once:            WriteStorage<'a, ScaleOnce>,
    pub health:                WriteStorage<'a, Health>,
    pub health_editor:         WriteStorage<'a, HealthEditor>,
    pub health_display:        WriteStorage<'a, HealthDisplay>,
    pub deals_damage:          WriteStorage<'a, DealsDamage>,
    pub takes_damage:          WriteStorage<'a, TakesDamage>,
}
