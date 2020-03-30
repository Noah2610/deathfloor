mod echo;
mod group;
mod move_action;

pub mod prelude {
    pub use super::echo::Echo;
    pub use super::group::Group;
    pub use super::move_action::MoveAction;
    pub use super::ActionTriggerStorages;
    pub use super::ActionType;
}

use super::action_trigger::ActionTrigger;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{SystemData, World, WriteStorage};
use prelude::*;

#[derive(Clone, Deserialize)]
pub enum ActionType {
    Echo(Echo),
    Group(Group),
    MoveAction(MoveAction),
}

#[derive(SystemData)]
pub struct ActionTriggerStorages<'a> {
    pub echo:        WriteStorage<'a, ActionTrigger<Echo>>,
    pub group:       WriteStorage<'a, ActionTrigger<Group>>,
    pub move_action: WriteStorage<'a, ActionTrigger<MoveAction>>,
}
