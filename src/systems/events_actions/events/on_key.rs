use super::system_prelude::*;
use deathframe::core::resources::input_manager::ActionState;

#[derive(Default)]
pub struct HandleEventOnKey;

impl<'a> System<'a> for HandleEventOnKey {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        Read<'a, InputManager<IngameBindings>>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            input_manager,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger) in
            (&events_register_store, &mut action_type_trigger_store).join()
        {
            for action_state in
                [ActionState::Down, ActionState::Up, ActionState::Pressed]
                    .iter()
            {
                input_manager.actions_for_each(*action_state, |key| {
                    if let Some(event_type) = get_event_type_from_action_state(
                        &action_state,
                        key.clone(),
                    ) {
                        if let Some(action) =
                            events_register.get_action(&event_type).cloned()
                        {
                            action_type_trigger.add_action(action);
                        }
                    }
                });
            }
        }
    }
}

fn get_event_type_from_action_state(
    action: &ActionState,
    key: IngameActionBinding,
) -> Option<EventType> {
    match action {
        ActionState::Down => Some(EventType::OnKeyDown(key)),
        ActionState::Up => Some(EventType::OnKeyUp(key)),
        ActionState::Pressed => Some(EventType::OnKeyPressed(key)),
        ActionState::None => None,
    }
}
