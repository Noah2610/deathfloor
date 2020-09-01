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
            for action_type in action_type_trigger.drain_actions() {
                match action_type {
                    ActionType::Echo(echo) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.echo)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(echo);
                        }
                    }
                    ActionType::Group(group) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.group)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(group);
                        }
                    }
                    ActionType::MoveAction(move_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.move_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(move_action);
                        }
                    }
                    ActionType::Random(random) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.random)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(random);
                        }
                    }
                    ActionType::Delay(delay) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.delay)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(delay);
                        }
                    }
                    ActionType::RepeatDelay(repeat_delay) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.repeat_delay)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(repeat_delay);
                        }
                    }
                    ActionType::InsertComponents(insert_components) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.insert_components)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(insert_components);
                        }
                    }
                    ActionType::HealthAction(health_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.health_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(health_action);
                        }
                    }
                    ActionType::AnimationAction(animation_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.animation_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(animation_action);
                        }
                    }
                    ActionType::SoundAction(sound_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.sound_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(sound_action);
                        }
                    }
                    ActionType::EntityAction(entity_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.entity_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(entity_action);
                        }
                    }
                    ActionType::SpawnAction(spawn_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.spawn_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(spawn_action);
                        }
                    }
                    ActionType::LifecycleAction(lifecycle_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.lifecycle_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(lifecycle_action);
                        }
                    }
                    ActionType::PlayerAction(player_action) => {
                        if let Some(action_trigger) =
                            (&mut action_trigger_components.player_action)
                                .get_mut_or_default(entity)
                        {
                            action_trigger.add_action(player_action);
                        }
                    }
                }
            }
        }
    }
}
