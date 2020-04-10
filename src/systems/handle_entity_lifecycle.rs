use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEntityLifecycleSystem;

impl<'a> System<'a> for HandleEntityLifecycleSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Lifecycle>,
        ReadStorage<'a, Health>,
    );

    fn run(
        &mut self,
        (entities, mut lifecycle_store, health_store): Self::SystemData,
    ) {
        for (entity, lifecycle, health_opt) in
            (&entities, &mut lifecycle_store, health_store.maybe()).join()
        {
            let is_prolonged = lifecycle.is_prolonged();
            match &lifecycle.state {
                LifecycleState::Initial => lifecycle.next_state().unwrap(),
                LifecycleState::Spawn => {
                    if !is_prolonged {
                        lifecycle.next_state().unwrap();
                    }
                }
                LifecycleState::Alive => {
                    if let Some(health) = health_opt {
                        if !health.is_alive() {
                            lifecycle.next_state().unwrap();
                        }
                    }
                }
                LifecycleState::Death => {
                    if !is_prolonged {
                        lifecycle.next_state().unwrap();
                    }
                }
                LifecycleState::Despawn => {
                    entities.delete(entity).expect(
                        "Couldn't delete entity with LifecycleState::Despawn",
                    );
                }
            }
            lifecycle.update();
        }
    }
}
