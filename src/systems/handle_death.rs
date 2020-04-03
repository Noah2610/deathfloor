use super::system_prelude::*;

#[derive(Default)]
pub struct HandleDeathSystem;

impl<'a> System<'a> for HandleDeathSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, Health>);

    fn run(&mut self, (entities, health_store): Self::SystemData) {
        for (entity, health) in (&entities, &health_store).join() {
            if !health.is_alive() {
                // TODO: Fire OnDeath event somehow
                entities.delete(entity).unwrap();
            }
        }
    }
}
