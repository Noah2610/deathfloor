use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnCollision;

impl<'a> System<'a> for HandleEventOnCollision {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            colliders,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, collider) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &colliders,
        )
            .join()
        {
            for (event, action) in events_register.events() {
                match event {
                    EventType::OnCollision(None)
                        if !collider.collisions.is_empty() =>
                    {
                        action_type_trigger.trigger(action.clone());
                    }
                    EventType::OnCollision(Some(query_exp)) => {
                        if collider
                            .query::<FindQuery<CollisionTag>>()
                            .exp(query_exp)
                            .run()
                            .is_some()
                        {
                            action_type_trigger.trigger(action.clone());
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}
