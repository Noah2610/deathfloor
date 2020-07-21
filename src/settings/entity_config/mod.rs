mod variants;

pub mod prelude {
    pub use super::variants::EntityConfigVariants;
    pub use super::EntityComponentsData;
    pub use super::EntityComponentsStorages;
    pub use super::EntityConfig;
}

use super::hitbox_config::HitboxConfig;
use crate::animation_key::AnimationKey;
use crate::collision_tag::CollisionTagWrapper;
use crate::components::prelude::*;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{Entities, SystemData, World, WriteStorage};
use variants::EntityConfigVariants;

/// Config for entities.
/// All fields are optional and can be omitted.
#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EntityConfig {
    /// List of components to be added to the entity.
    pub components:    Option<EntityComponentsData>,
    /// Variants for this entity.
    pub variants:      Option<EntityConfigVariants>,
    /// Register events/actions.
    pub events:        Option<EventsRegister>,
    /// General collision tag config.
    pub collision_tag: Option<CollisionTagWrapper>,
    /// Solid collision tag config.
    pub solid_tag:     Option<CollisionTagWrapper>,
}

impl Merge for EntityConfig {
    /// `other` takes precedence.
    fn merge(&mut self, other: Self) {
        *self = Self {
            components:    self.components.take().merged(other.components),
            variants:      self.variants.take().merged(other.variants),
            events:        self.events.take().merged(other.events),
            collision_tag: other.collision_tag.or(self.collision_tag.take()),
            solid_tag:     other.solid_tag.or(self.solid_tag.take()),
        };
    }
}

/// List of posible components for an entity.
/// All can optionally be added to an entity's config.
#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EntityComponentsData {
    pub size:                  Option<Size>,
    pub velocity:              Option<Velocity>,
    pub gravity:               Option<Gravity>,
    pub max_movement_velocity: Option<MaxMovementVelocity>,
    pub base_friction:         Option<BaseFriction>,
    pub animation:             Option<Animation>,
    pub animations:            Option<AnimationsContainer<AnimationKey>>,
    pub hitbox:                Option<HitboxConfig>,
    pub walker:                Option<Walker>,
    pub jumppad:               Option<Jumppad>,
    pub scale_once:            Option<ScaleOnce>,
    pub health:                Option<Health>,
    pub health_display:        Option<HealthDisplay>,
    pub deals_damage:          Option<DealsDamage>,
    pub takes_damage:          Option<TakesDamage>,
    pub bullet:                Option<Bullet>,
    #[serde(rename = "ledge_detector")]
    pub ledge_detector_data:   Option<LedgeDetectorData>,
}

impl Merge for EntityComponentsData {
    /// `other` takes precedence.
    #[rustfmt::skip]
    fn merge(&mut self, other: EntityComponentsData) {
        *self = Self {
            size:                  other.size.or(self.size.take()),
            velocity:              other.velocity.or(self.velocity.take()),
            gravity:               other.gravity.or(self.gravity.take()),
            max_movement_velocity: other.max_movement_velocity.or(self.max_movement_velocity.take()),
            base_friction:         other.base_friction.or(self.base_friction.take()),
            animation:             other.animation.or(self.animation.take()),
            animations:            other.animations.or(self.animations.take()),
            hitbox:                other.hitbox.or(self.hitbox.take()),
            walker:                other.walker.or(self.walker.take()),
            jumppad:               other.jumppad.or(self.jumppad.take()),
            scale_once:            other.scale_once.or(self.scale_once.take()),
            health:                other.health.or(self.health.take()),
            health_display:        other.health_display.or(self.health_display.take()),
            deals_damage:          other.deals_damage.or(self.deals_damage.take()),
            takes_damage:          other.takes_damage.or(self.takes_damage.take()),
            bullet:                other.bullet.or(self.bullet.take()),
            ledge_detector_data:   other.ledge_detector_data.or(self.ledge_detector_data.take()),
        };
    }
}

#[derive(SystemData)]
pub struct EntityComponentsStorages<'a> {
    pub entities:                       Entities<'a>,
    pub transform:                      WriteStorage<'a, Transform>,
    pub size:                           WriteStorage<'a, Size>,
    pub velocity:                       WriteStorage<'a, Velocity>,
    pub gravity:                        WriteStorage<'a, Gravity>,
    pub max_movement_velocity:          WriteStorage<'a, MaxMovementVelocity>,
    pub base_friction:                  WriteStorage<'a, BaseFriction>,
    pub animation:                      WriteStorage<'a, Animation>,
    pub animations: WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    pub animation_editor:               WriteStorage<'a, AnimationEditor>,
    pub hitbox:                         WriteStorage<'a, Hitbox>,
    pub walker:                         WriteStorage<'a, Walker>,
    pub jumppad:                        WriteStorage<'a, Jumppad>,
    pub jumppad_affected:               WriteStorage<'a, JumppadAffected>,
    pub scale_once:                     WriteStorage<'a, ScaleOnce>,
    pub health:                         WriteStorage<'a, Health>,
    pub health_action_queue:            WriteStorage<'a, HealthActionQueue>,
    pub health_display:                 WriteStorage<'a, HealthDisplay>,
    pub deals_damage:                   WriteStorage<'a, DealsDamage>,
    pub takes_damage:                   WriteStorage<'a, TakesDamage>,
    pub bullet:                         WriteStorage<'a, Bullet>,
    pub ledge_detector:                 WriteStorage<'a, LedgeDetector>,
    pub ledge_detector_corner_detector:
        WriteStorage<'a, LedgeDetectorCornerDetector>,
    pub collider_solid:                 WriteStorage<'a, Collider<SolidTag>>,
    pub solid:                          WriteStorage<'a, Solid<SolidTag>>,
}
