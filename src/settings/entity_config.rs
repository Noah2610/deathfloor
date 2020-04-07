pub mod prelude {
    pub use super::EntityComponentsData;
    pub use super::EntityComponentsStorages;
    pub use super::EntityConfig;
}

use super::hitbox_config::HitboxConfig;
use crate::animation_key::AnimationKey;
use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::*;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{SystemData, World, WriteStorage};

/// Config for entities.
/// All fields are optional and can be omitted.
#[derive(Clone, Deserialize)]
pub struct EntityConfig {
    /// List of components to be added to the entity.
    pub components:    Option<EntityComponentsData>,
    /// Register events/actions.
    pub events:        Option<EventsRegister>,
    /// General collision tag config.
    pub collision_tag: Option<CollisionTagWrapper>,
    /// Solid collision tag config.
    pub solid_tag:     Option<CollisionTagWrapper>,
}

/// List of posible components for an entity.
/// All can optionally be added to an entity's config.
#[derive(Clone, Deserialize)]
pub struct EntityComponentsData {
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
pub struct EntityComponentsStorages<'a> {
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
