use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnCollision;

impl<'a> System<'a> for HandleEventOnCollision {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            colliders,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (
            events_register,
            action_type_trigger,
            collider,
            loadable_opt,
            loaded_opt,
        ) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &colliders,
            loadable_store.maybe(),
            loaded_store.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                for (event, action) in events_register.events() {
                    match event {
                        EventType::OnCollision(None)
                            if !collider.collisions.is_empty() =>
                        {
                            action_type_trigger.add_action(action.clone());
                        }
                        EventType::OnCollision(Some(query_exp)) => {
                            if collider
                                .query::<FindQuery<CollisionTag>>()
                                .exp(query_exp)
                                .run()
                                .is_some()
                            {
                                action_type_trigger.add_action(action.clone());
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}
