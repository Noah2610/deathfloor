use super::system_prelude::*;
use action::ActionType;
use deathframe::amethyst::ecs::storage::GenericWriteStorage;

#[derive(Default)]
pub(super) struct TriggerActionsFromActionTypesSystem;

impl<'a> System<'a> for TriggerActionsFromActionTypesSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTypeTrigger>,
        action::ActionTriggerStorages<'a>,
    );

    fn run(
        &mut self,
        (
                entities,
                mut action_type_triggers,
                mut action_trigger_components,
            ): Self::SystemData,
    ) {
        for (entity, action_type_trigger) in
            (&entities, &mut action_type_triggers).join()
        {
            for action_type in action_type_trigger.drain() {
                match action_type {
                    ActionType::Echo(echo) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.echo)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.trigger(echo);
                        }
                    }
                    ActionType::Group(group) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.group)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.trigger(group);
                        }
                    }
                    ActionType::MoveAction(move_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.move_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.trigger(move_action);
                        }
                    }
                    ActionType::Random(random) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.random)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.trigger(random);
                        }
                    }
                }
            }
        }
    }
}
