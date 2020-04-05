use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventLifecycle;

impl<'a> System<'a> for HandleEventLifecycle {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Lifecycle>,
    );

    fn run(
        &mut self,
        (
            entities,
            events_register_store,
            mut action_type_trigger_store,
            lifecyle_store,
        ): Self::SystemData,
    ) {
        for (entity, events_register, action_type_trigger, lifecycle) in (
            &entities,
            &events_register_store,
            &mut action_type_trigger_store,
            &lifecyle_store,
        )
            .join()
        {
            let event_type = EventType::Lifecycle(lifecycle.state.clone());
            if let Some(action) =
                events_register.get_action(&event_type).cloned()
            {
                action_type_trigger.trigger(action);
            }
        }
    }
}
