use super::system_prelude::*;

#[derive(Default)]
pub struct HandleDyingEntitiesSystem;

impl<'a> System<'a> for HandleDyingEntitiesSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Lifecycle>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, Collidable<CollisionTag>>,
        WriteStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, EventsRegister>,
        WriteStorage<'a, LedgeDetector>,
    );

    fn run(
        &mut self,
        (
            entities,
            lifecycle_store,
            mut velocity_store,
            mut gravity_store,
            mut collider_store,
            mut collidable_store,
            mut solid_store,
            mut events_register_store,
            mut ledge_detector_store,
        ): Self::SystemData,
    ) {
        for (entity, lifecycle) in (&entities, &lifecycle_store).join() {
            if let LifecycleState::Death = &lifecycle.state {
                velocity_store.remove(entity);
                gravity_store.remove(entity);
                collider_store.remove(entity);
                collidable_store.remove(entity);
                solid_store.remove(entity);
                events_register_store.remove(entity);
                ledge_detector_store.remove(entity);
            }
        }
    }
}
