use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventLifecycle;

impl<'a> System<'a> for HandleEventLifecycle {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Lifecycle>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            events_register_store,
            mut action_type_trigger_store,
            lifecyle_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (
            _entity,
            events_register,
            action_type_trigger,
            lifecycle,
            loadable_opt,
            loaded_opt,
        ) in (
            &entities,
            &events_register_store,
            &mut action_type_trigger_store,
            &lifecyle_store,
            loadable_store.maybe(),
            loaded_store.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                let event_type = EventType::Lifecycle(lifecycle.state.clone());
                if let Some(action) =
                    events_register.get_action(&event_type).cloned()
                {
                    action_type_trigger.add_action(action);
                }
            }
        }
    }
}
