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
        {}
    }
}
