mod actions;
mod events;

pub use bundle::EventHandlersBundle;

use super::system_prelude;

mod trigger_actions_from_action_types_system {
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
            for (entity, mut action_type_trigger) in
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
                    }
                }
            }
        }
    }
}

mod bundle {
    use super::actions;
    use super::events;
    use super::trigger_actions_from_action_types_system::TriggerActionsFromActionTypesSystem as TriggerActions;
    use amethyst::core::bundle::SystemBundle;
    use amethyst::ecs::{DispatcherBuilder, World};
    use deathframe::core::amethyst;

    pub struct EventHandlersBundle<'a> {
        deps: &'a [&'a str],
    }

    impl<'a> EventHandlersBundle<'a> {
        pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
            self.deps = deps;
            self
        }
    }

    impl<'a, 'b, 'c> SystemBundle<'a, 'b> for EventHandlersBundle<'c> {
        fn build(
            self,
            _world: &mut World,
            builder: &mut DispatcherBuilder<'a, 'b>,
        ) -> Result<(), amethyst::Error> {
            // EVENTS
            builder.add(
                events::on_spawn::HandleEventOnSpawn::default(),
                "handle_event_on_spawn_system",
                self.deps,
            );
            builder.add(
                events::on_collision::HandleEventOnCollision::default(),
                "handle_event_on_collision_system",
                self.deps,
            );
            builder.add(
                events::interval::HandleEventInterval::default(),
                "handle_event_interval_system",
                self.deps,
            );

            // TRIGGER ACTIONS FROM ACTION TYPES
            builder.add(
                TriggerActions::default(),
                "trigger_actions_from_action_types_system",
                &[self.deps, &[
                    "handle_event_on_spawn_system",
                    "handle_event_on_collision_system",
                    "handle_event_interval_system",
                ]]
                .concat(),
            );

            // ACTIONS
            let actions_deps =
                &[self.deps, &["trigger_actions_from_action_types_system"]]
                    .concat();

            builder.add(
                actions::echo::HandleActionEcho::default(),
                "handle_action_echo_system",
                actions_deps,
            );
            builder.add(
                actions::group::HandleActionGroup::default(),
                "handle_action_group_system",
                actions_deps,
            );
            builder.add(
                actions::move_action::HandleActionMoveAction::default(),
                "handle_action_move_action_system",
                actions_deps,
            );
            Ok(())
        }
    }

    impl<'a> Default for EventHandlersBundle<'a> {
        fn default() -> Self {
            Self {
                deps: Default::default(),
            }
        }
    }
}
