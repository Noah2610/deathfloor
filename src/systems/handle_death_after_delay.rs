use super::system_prelude::*;

#[derive(Default)]
pub struct HandleDeathAfterDelaySystem;

impl<'a> System<'a> for HandleDeathAfterDelaySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, DeathAfterDelay>,
        WriteStorage<'a, Lifecycle>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut death_after_delay_store,
            mut lifecycle_store,
        ): Self::SystemData,
    ) {
        for (entity, death_after_delay, lifecycle_opt) in (
            &entities,
            &mut death_after_delay_store,
            (&mut lifecycle_store).maybe(),
        )
            .join()
        {
            if death_after_delay.timer.state.is_stopped() {
                death_after_delay.timer.start().unwrap();
            }
            death_after_delay.timer.update().unwrap();
            if death_after_delay.timer.state.is_finished() {
                if let Some(lifecycle) = lifecycle_opt {
                    lifecycle.state = LifecycleState::Death;
                } else {
                    entities.delete(entity).unwrap();
                }
            }
        }
    }
}
