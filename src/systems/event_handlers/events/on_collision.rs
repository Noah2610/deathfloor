use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnCollision;

impl<'a> System<'a> for HandleEventOnCollision {
    type SystemData = (
        WriteStorage<'a, EventListener>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(&mut self, (mut event_listeners, colliders): Self::SystemData) {
        for (event_listener, collider) in
            (&mut event_listeners, &colliders).join()
        {
            // TODO: This doesn't seem right...
            let mut events_to_trigger: Vec<EventType> = Vec::new();

            for event in event_listener.events() {
                match event {
                    EventType::OnCollision(None)
                        if !collider.collisions.is_empty() =>
                    {
                        events_to_trigger.push(event.clone());
                    }
                    EventType::OnCollision(Some(query_exp)) => {
                        if collider
                            .query::<FindQuery<CollisionTag>>()
                            .exp(query_exp)
                            .run()
                            .is_some()
                        {
                            events_to_trigger.push(event.clone());
                        }
                    }
                    _ => (),
                }
            }

            for event in events_to_trigger {
                event_listener.trigger(&event);
            }
        }
    }
}
