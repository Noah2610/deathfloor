use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventInit;

impl<'a> System<'a> for HandleEventInit {
    type SystemData = (
        WriteStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
    );

    fn run(
        &mut self,
        (
            mut events_register_store,
            mut action_type_trigger_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger) in
            (&mut events_register_store, &mut action_type_trigger_store).join()
        {
            if !events_register.data.init.initialized {
                events_register.data.init.initialized = true;
                if let Some(action) =
                    events_register.get_action(&EventType::Init)
                {
                    action_type_trigger.add_action(action.clone());
                }
            }
        }
    }
}
