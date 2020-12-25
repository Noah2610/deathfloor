use super::actions;
use super::events;
use super::trigger_actions_from_action_types::TriggerActionsFromActionTypesSystem as TriggerActions;
use amethyst::core::bundle::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use deathframe::core::amethyst;

pub struct EventsActionsBundle<'a> {
    deps: &'a [&'a str],
}

impl<'a> EventsActionsBundle<'a> {
    pub fn with_deps(mut self, deps: &'a [&'a str]) -> Self {
        self.deps = deps;
        self
    }
}

impl<'a, 'b, 'c> SystemBundle<'a, 'b> for EventsActionsBundle<'c> {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst::Error> {
        // EVENTS
        builder.add(
            events::lifecycle::HandleEventLifecycle::default(),
            "handle_event_lifecycle_system",
            &[self.deps, &["update_lifecycle_system"]].concat(),
        );
        builder.add(
            events::on_collision::HandleEventOnCollision::default(),
            "handle_event_on_collision_system",
            &[self.deps, &["update_collisions_system"]].concat(),
        );
        builder.add(
            events::delay::HandleEventDelay::default(),
            "handle_event_delay_system",
            self.deps,
        );
        builder.add(
            events::interval::HandleEventInterval::default(),
            "handle_event_interval_system",
            self.deps,
        );
        builder.add(
            events::on_ledge_detect::HandleEventOnLedgeDetectSystem::default(),
            "handle_event_on_ledge_detect_system",
            &[self.deps, &["handle_ledge_detector_system"]].concat(),
        );
        builder.add(
            events::init::HandleEventInit::default(),
            "handle_event_init_system",
            self.deps,
        );
        builder.add(
            events::on_animation_end::HandleEventOnAnimationEnd::default(),
            "handle_event_on_animation_end_system",
            self.deps,
        );
        builder.add(
            events::on_interact::HandleEventOnInteract::default(),
            "handle_event_on_interact_system",
            self.deps,
        );
        builder.add(
            events::on_key::HandleEventOnKey::default(),
            "handle_event_on_key_system",
            self.deps,
        );
        builder.add(
            events::on_shoot::HandleEventOnShoot::default(),
            "handle_event_on_shoot_system",
            self.deps,
        );
        builder.add(
            events::jump_events::HandleEventJumpEvents::default(),
            "handle_event_jump_events_system",
            self.deps,
        );

        // TRIGGER ACTIONS FROM ACTION TYPES
        builder.add(
            TriggerActions::default(),
            "trigger_actions_from_action_types_system",
            &[self.deps, &[
                "handle_event_lifecycle_system",
                "handle_event_on_collision_system",
                "handle_event_delay_system",
                "handle_event_interval_system",
                "handle_event_on_ledge_detect_system",
                "handle_event_init_system",
                "handle_event_on_animation_end_system",
                "handle_event_on_interact_system",
                "handle_event_on_key_system",
                "handle_event_on_shoot_system",
                "handle_event_jump_events_system",
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
        builder.add(
            actions::insert_components::HandleActionInsertComponents::default(),
            "handle_action_insert_components_system",
            actions_deps,
        );
        builder.add(
            actions::health_action::HandleActionHealthAction::default(),
            "handle_action_health_action_system",
            actions_deps,
        );
        builder.add(
            actions::animation_action::HandleActionAnimationAction::default(),
            "handle_action_animation_action_system",
            actions_deps,
        );
        builder.add(
            actions::sound_action::HandleActionSoundAction::default(),
            "handle_action_sound_action_system",
            actions_deps,
        );
        builder.add(
            actions::entity_action::HandleActionEntityAction::default(),
            "handle_action_entity_action_system",
            actions_deps,
        );
        builder.add(
            actions::spawn_action::HandleActionSpawnAction::default(),
            "handle_action_spawn_action_system",
            actions_deps,
        );
        builder.add(
            actions::lifecycle_action::HandleActionLifecycleAction::default(),
            "handle_action_lifecycle_action_system",
            actions_deps,
        );
        builder.add(
            actions::player_action::HandleActionPlayerAction::default(),
            "handle_action_player_action_system",
            actions_deps,
        );
        builder.add(
            actions::if_action::HandleActionIfAction::default(),
            "handle_action_if_action_system",
            actions_deps,
        );
        builder.add(
            actions::call::HandleActionCall::default(),
            "handle_action_call_system",
            actions_deps,
        );
        builder.add(
            actions::control_action::HandleActionControlAction::default(),
            "handle_action_control_action_system",
            actions_deps,
        );
        builder.add(
            actions::variable_action::HandleActionVariableAction::default(),
            "handle_action_variable_action_system",
            actions_deps,
        );
        builder.add(
            actions::variable_action::HandleUpdateVariableRegister::default(),
            "handle_update_variable_register_system",
            &["handle_action_variable_action_system"],
        );
        builder.add(
            actions::foreign_entity_action::HandleActionForeignEntityAction::default(),
            "handle_action_foreign_entity_action_system",
            actions_deps,
        );
        Ok(())
    }
}

impl<'a> Default for EventsActionsBundle<'a> {
    fn default() -> Self {
        Self {
            deps: Default::default(),
        }
    }
}
