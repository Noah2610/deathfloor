use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionCall;

impl<'a> System<'a> for HandleActionCall {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::Call>>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, EventsRegister>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut action_type_trigger_store,
            events_register_store,
        ): Self::SystemData,
    ) {
        for (action_trigger, action_type_trigger, events_register) in (
            &mut action_trigger_store,
            &mut action_type_trigger_store,
            &events_register_store,
        )
            .join()
        {
            for call_action in action_trigger.drain_actions() {
                if let Some(action) = events_register
                    .get_action(&EventType::Function(call_action.0.clone()))
                {
                    action_type_trigger.add_action(action.clone());
                } else {
                    eprintln!(
                        "[WARNING]\n    Tried to Call undefined Function \
                         \"{}\"",
                        call_action.0,
                    );
                }
            }
        }
    }
}
