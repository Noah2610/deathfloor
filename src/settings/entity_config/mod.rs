mod inheritance_chain;
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
use crate::settings::prelude::AbstractEntitiesSettings;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{Entities, SystemData, World, WriteStorage};
use inheritance_chain::EntityConfigInheritanceChain;
use std::collections::HashMap;
use variants::EntityConfigVariants;

/// Config for entities.
/// All fields are optional and can be omitted.
#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct EntityConfig {
    /// Inherit / merge entity configs from the given abstract configs.
    /// Merges configs from left to right, meaning later entries overwrite earlier ones.
    pub inherits:        Option<EntityConfigInheritanceChain>,
    /// List of components to be added to the entity.
    pub components:      Option<EntityComponentsData>,
    /// Variants for this entity.
    pub variants:        Option<EntityConfigVariants>,
    /// The default variant to use, when no variant prop was set.
    pub default_variant: Option<String>,
    /// Register events/actions.
    pub events:          Option<EventsRegister>,
    /// General collision tag config.
    pub collision_tag:   Option<CollisionTagWrapper>,
    /// Solid collision tag config.
    pub solid_tag:       Option<CollisionTagWrapper>,
}

impl EntityConfig {
    /// Every inherited abstract entity config defined
    /// is merged together here.
    /// All variants' inheritance chains are also merged together.
    /// This function should only be called once.
    pub fn with_inheritance(
        mut self,
        abstract_entities_settings: &AbstractEntitiesSettings,
    ) -> Self {
        // Merge root inheritance chain
        if let Some(inheritance_chain) = &self.inherits {
            self = inheritance_chain
                .generate_entity_config(abstract_entities_settings)
                .merged(self);
        }

        // Merge variants' inheritance chains
        if let Some(variants) = self.variants {
            let new_variants: EntityConfigVariants = variants
                .variants
                .into_iter()
                .map(|(variant_name, variant)| {
                    (
                        variant_name,
                        variant.with_inheritance(abstract_entities_settings),
                    )
                })
                .collect::<HashMap<_, _>>()
                .into();
            self.variants = Some(new_variants);
        }

        self
    }

    /// Merge the given variant name into this entity config.
    /// Returns the variant that has been merged.
    pub fn merge_variant(
        &mut self,
        variant_opt: Option<String>,
    ) -> Option<(String, EntityConfig)> {
        let variant_name = variant_opt.or(self.default_variant.clone());
        let mut merged_variant = None;

        // Merge variant into entity config
        if let Some(variant_name) = variant_name {
            if let Some(variant) = {
                self.variants
                    .as_ref()
                    .and_then(|variants| variants.get(&variant_name).cloned())
            } {
                merged_variant = Some((variant_name, variant.clone()));
                self.merge(variant);
            }
        }

        merged_variant
    }
}

impl Merge for EntityConfig {
    /// `other` takes precedence.
    #[rustfmt::skip]
    fn merge(&mut self, other: Self) {
        *self = Self {
            inherits:        self.inherits.take().merged(other.inherits),
            components:      self.components.take().merged(other.components),
            variants:        self.variants.take().merged(other.variants),
            default_variant: other.default_variant.or(self.default_variant.take()),
            events:          self.events.take().merged(other.events),
            collision_tag:   other.collision_tag.or(self.collision_tag.take()),
            solid_tag:       other.solid_tag.or(self.solid_tag.take()),
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
    pub movement_acceleration: Option<MovementAcceleration>,
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
    pub death_on_contact:      Option<DeathOnContact>,
    pub death_after_delay:     Option<DeathAfterDelay>,
    pub interactable:          Option<Interactable>,
    pub facing:                Option<Facing>,
    pub jumper:                Option<Jumper>,
    pub wall_jumper:           Option<WallJumper>,
    pub wall_slider:           Option<WallSlider>,
    pub shooter:               Option<Shooter>,
    pub kill_velocity_min:     Option<KillVelocityMin>,
    pub solid_pusher:          Option<SolidPusher>,
    pub solid_pushable:        Option<SolidPushable>,
    pub non_precise_movement:  Option<NonPreciseMovement>,
    pub loader:                Option<Loader>,
    pub loadable:              Option<Loadable>,
    pub unloaded:              Option<Unloaded>,
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
            movement_acceleration: other.movement_acceleration.or(self.movement_acceleration.take()),
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
            death_on_contact:      other.death_on_contact.or(self.death_on_contact.take()),
            death_after_delay:     other.death_after_delay.or(self.death_after_delay.take()),
            interactable:          other.interactable.or(self.interactable.take()),
            facing:                other.facing.or(self.facing.take()),
            jumper:                other.jumper.or(self.jumper.take()),
            wall_jumper:           other.wall_jumper.or(self.wall_jumper.take()),
            wall_slider:           other.wall_slider.or(self.wall_slider.take()),
            shooter:               other.shooter.or(self.shooter.take()),
            kill_velocity_min:     other.kill_velocity_min.or(self.kill_velocity_min.take()),
            solid_pusher:          other.solid_pusher.or(self.solid_pusher.take()),
            solid_pushable:        other.solid_pushable.or(self.solid_pushable.take()),
            non_precise_movement:  other.non_precise_movement.or(self.non_precise_movement.take()),
            loader:                other.loader.or(self.loader.take()),
            loadable:              other.loadable.or(self.loadable.take()),
            unloaded:              other.unloaded.or(self.unloaded.take()),
        };
    }
}

#[derive(SystemData)]
#[rustfmt::skip]
pub struct EntityComponentsStorages<'a> {
    pub entities:                       Entities<'a>,
    pub transform:                      WriteStorage<'a, Transform>,
    pub size:                           WriteStorage<'a, Size>,
    pub velocity:                       WriteStorage<'a, Velocity>,
    pub gravity:                        WriteStorage<'a, Gravity>,
    pub max_movement_velocity:          WriteStorage<'a, MaxMovementVelocity>,
    pub movement_acceleration:          WriteStorage<'a, MovementAcceleration>,
    pub base_friction:                  WriteStorage<'a, BaseFriction>,
    pub animation:                      WriteStorage<'a, Animation>,
    pub animations:                     WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    pub animation_editor:               WriteStorage<'a, AnimationEditor>,
    pub hitbox:                         WriteStorage<'a, Hitbox>,
    pub collider:                       WriteStorage<'a, Collider<CollisionTag>>,
    pub collidable:                     WriteStorage<'a, Collidable<CollisionTag>>,
    pub solid:                          WriteStorage<'a, Solid<SolidTag>>,
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
    pub ledge_detector_corner_detector: WriteStorage<'a, LedgeDetectorCornerDetector>,
    pub follow:                         WriteStorage<'a, Follow>,
    pub death_bound:                    WriteStorage<'a, DeathBound>,
    pub death_on_contact:               WriteStorage<'a, DeathOnContact>,
    pub death_after_delay:              WriteStorage<'a, DeathAfterDelay>,
    pub interactable:                   WriteStorage<'a, Interactable>,
    pub facing:                         WriteStorage<'a, Facing>,
    pub jumper:                         WriteStorage<'a, Jumper>,
    pub wall_jumper:                    WriteStorage<'a, WallJumper>,
    pub wall_slider:                    WriteStorage<'a, WallSlider>,
    pub shooter:                        WriteStorage<'a, Shooter>,
    pub kill_velocity_min:              WriteStorage<'a, KillVelocityMin>,
    pub solid_pusher:                   WriteStorage<'a, SolidPusher>,
    pub solid_pushable:                 WriteStorage<'a, SolidPushable>,
    pub non_precise_movement:           WriteStorage<'a, NonPreciseMovement>,
    pub loader:                         WriteStorage<'a, Loader>,
    pub loadable:                       WriteStorage<'a, Loadable>,
    pub unloaded:                       WriteStorage<'a, Unloaded>,
}
