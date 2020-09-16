use crate::components::prelude::*;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{Entities, ReadStorage, SystemData, World};

#[derive(SystemData)]
pub struct ConditionStorages<'a> {
    pub entities:  Entities<'a>,
    pub transform: ReadStorage<'a, Transform>,
    pub velocity:  ReadStorage<'a, Velocity>,
    pub health:    ReadStorage<'a, Health>,
    pub player:    ReadStorage<'a, Player>,
    pub facing:    ReadStorage<'a, Facing>,
}
