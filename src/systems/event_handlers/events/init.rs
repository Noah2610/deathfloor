use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventInit;

impl<'a> System<'a> for HandleEventInit {
    type SystemData = (
        WriteStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            mut events_register_store,
            mut action_type_trigger_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, loadable_opt, loaded_opt) in
            (
                &mut events_register_store,
                &mut action_type_trigger_store,
                loadable_store.maybe(),
                loaded_store.maybe(),
            )
                .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
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
}
