use super::system_prelude::*;
use std::collections::HashSet;

/// Removes select components from entities whose `Lifecycle` is `Death`.
/// Stages removal of components for `Death` entities,
/// but only actually removes the components in the next frame.
/// This is primarily to make the `EventsRegister`'s `OnDeath` event trigger.
#[derive(Default)]
pub struct HandleDyingEntitiesSystem {
    to_remove: HashSet<Entity>,
}

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
        for entity in self.to_remove.drain() {
            if entities.is_alive(entity) {
                velocity_store.remove(entity);
                gravity_store.remove(entity);
                collider_store.remove(entity);
                collidable_store.remove(entity);
                solid_store.remove(entity);
                events_register_store.remove(entity);
                ledge_detector_store.remove(entity);
            }
        }

        for (entity, lifecycle) in (&entities, &lifecycle_store).join() {
            if let LifecycleState::Death = &lifecycle.state {
                self.to_remove.insert(entity);
            }
        }
    }
}
