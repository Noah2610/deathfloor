pub mod conditionals;

mod animation_action;
mod call;
mod control_action;
mod delay;
mod echo;
mod entity_action;
mod group;
mod health_action;
mod insert_components;
mod lifecycle_action;
mod move_action;
mod player_action;
mod random;
mod repeat_delay;
mod sound_action;
mod spawn_action;

pub mod prelude {
    pub use super::animation_action::AnimationAction;
    pub use super::call::Call;
    pub use super::conditionals;
    pub use super::control_action::ControlAction;
    pub use super::delay::Delay;
    pub use super::echo::Echo;
    pub use super::entity_action::EntityAction;
    pub use super::group::Group;
    pub use super::health_action::HealthAction;
    pub use super::insert_components::InsertComponents;
    pub use super::lifecycle_action::LifecycleAction;
    pub use super::move_action::MoveAction;
    pub use super::player_action::PlayerAction;
    pub use super::random::Random;
    pub use super::repeat_delay::RepeatDelay;
    pub use super::sound_action::SoundAction;
    pub use super::spawn_action::SpawnAction;
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
    Random(Random),
    Delay(Delay),
    RepeatDelay(RepeatDelay),
    InsertComponents(InsertComponents),
    HealthAction(HealthAction),
    AnimationAction(AnimationAction),
    SoundAction(SoundAction),
    EntityAction(EntityAction),
    SpawnAction(SpawnAction),
    LifecycleAction(LifecycleAction),
    PlayerAction(PlayerAction),
    Call(Call),
    If(conditionals::IfAction),
    ControlAction(ControlAction),
}

#[derive(SystemData)]
pub struct ActionTriggerStorages<'a> {
    pub echo:              WriteStorage<'a, ActionTrigger<Echo>>,
    pub group:             WriteStorage<'a, ActionTrigger<Group>>,
    pub move_action:       WriteStorage<'a, ActionTrigger<MoveAction>>,
    pub random:            WriteStorage<'a, ActionTrigger<Random>>,
    pub delay:             WriteStorage<'a, ActionTrigger<Delay>>,
    pub repeat_delay:      WriteStorage<'a, ActionTrigger<RepeatDelay>>,
    pub insert_components: WriteStorage<'a, ActionTrigger<InsertComponents>>,
    pub health_action:     WriteStorage<'a, ActionTrigger<HealthAction>>,
    pub animation_action:  WriteStorage<'a, ActionTrigger<AnimationAction>>,
    pub sound_action:      WriteStorage<'a, ActionTrigger<SoundAction>>,
    pub entity_action:     WriteStorage<'a, ActionTrigger<EntityAction>>,
    pub spawn_action:      WriteStorage<'a, ActionTrigger<SpawnAction>>,
    pub lifecycle_action:  WriteStorage<'a, ActionTrigger<LifecycleAction>>,
    pub player_action:     WriteStorage<'a, ActionTrigger<PlayerAction>>,
    pub call:              WriteStorage<'a, ActionTrigger<Call>>,
    pub if_action: WriteStorage<'a, ActionTrigger<conditionals::IfAction>>,
    pub control_action:    WriteStorage<'a, ActionTrigger<ControlAction>>,
}
