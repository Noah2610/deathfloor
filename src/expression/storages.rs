use crate::animation_key::AnimationKey;
use crate::components::prelude::*;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{Entities, ReadStorage, SystemData, World};

#[derive(SystemData)]
pub struct ExpressionStorages<'a> {
    pub entities:               Entities<'a>,
    pub transform:              ReadStorage<'a, Transform>,
    pub velocity:               ReadStorage<'a, Velocity>,
    pub health:                 ReadStorage<'a, Health>,
    pub player:                 ReadStorage<'a, Player>,
    pub facing:                 ReadStorage<'a, Facing>,
    pub entity_config_register: ReadStorage<'a, EntityConfigRegister>,
    pub collider:               ReadStorage<'a, Collider<CollisionTag>>,
    pub animations: ReadStorage<'a, AnimationsContainer<AnimationKey>>,
    pub variable_register:      ReadStorage<'a, VariableRegister>,
}
