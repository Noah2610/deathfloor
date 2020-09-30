use crate::animation_key::AnimationKey;
use crate::components::prelude::*;
use crate::settings::prelude::HitboxConfig;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{Entities, SystemData, World, WriteStorage};

/// Contains all possible, configurable components for this entity.
/// Given as an array of entity config components.
#[derive(Clone, Deserialize, Default)]
#[serde(deny_unknown_fields, from = "Vec<EntityConfigComponent>")]
pub struct EntityConfigComponents {
    pub components: Vec<EntityConfigComponent>,
}

impl Merge for EntityConfigComponents {
    /// `other` takes precedence.
    fn merge(&mut self, mut other: EntityConfigComponents) {
        self.components.append(&mut other.components);
    }
}

impl From<Vec<EntityConfigComponent>> for EntityConfigComponents {
    fn from(components: Vec<EntityConfigComponent>) -> Self {
        Self { components }
    }
}

/// Every possible entity config component.
#[derive(Clone, Deserialize)]
pub enum EntityConfigComponent {
    Velocity(Option<Velocity>),
    Gravity(Option<Gravity>),
    MaxMovementVelocity(Option<MaxMovementVelocity>),
    MovementAcceleration(Option<MovementAcceleration>),
    BaseFriction(Option<BaseFriction>),
    Animation(Option<Animation>),
    Animations(Option<AnimationsContainer<AnimationKey>>),
    Hitbox(Option<HitboxConfig>),
    Walker(Option<Walker>),
    Jumppad(Option<Jumppad>),
    ScaleOnce(Option<ScaleOnce>),
    Health(Option<Health>),
    HealthDisplay(Option<HealthDisplay>),
    DealsDamage(Option<DealsDamage>),
    TakesDamage(Option<TakesDamage>),
    Bullet(Option<Bullet>),
    LedgeDetector(Option<LedgeDetectorData>),
    DeathOnContact(Option<DeathOnContact>),
    DeathAfterDelay(Option<DeathAfterDelay>),
    Interactable(Option<Interactable>),
    Facing(Option<Facing>),
    Jumper(Option<Jumper>),
    WallJumper(Option<WallJumper>),
    WallSlider(Option<WallSlider>),
    Shooter(Option<Shooter>),
    KillVelocityMin(Option<KillVelocityMin>),
    SolidPusher(Option<SolidPusher>),
    SolidPushable(Option<SolidPushable>),
    NonPreciseMovement(Option<NonPreciseMovement>),
    Loader(Option<Loader>),
    Loadable(Option<Loadable>),
    Unloaded(Option<Unloaded>),
}

#[derive(SystemData)]
#[rustfmt::skip]
pub struct EntityConfigComponentsStorages<'a> {
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
