use super::actions;
use super::events;
use super::trigger_actions_from_action_types::TriggerActionsFromActionTypesSystem as TriggerActions;
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
        builder.add(
            actions::random::HandleActionRandom::default(),
            "handle_action_random_system",
            actions_deps,
        );
        builder.add(
            actions::delay::HandleActionDelay::default(),
            "handle_action_delay_system",
            actions_deps,
        );
        builder.add(
            actions::repeat_delay::HandleActionRepeatDelay::default(),
            "handle_action_repeat_delay_system",
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
