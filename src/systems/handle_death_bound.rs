use super::system_prelude::*;

#[derive(Default)]
pub struct HandleDeathBoundSystem;

impl<'a> System<'a> for HandleDeathBoundSystem {
    type SystemData = (Entities<'a>, ReadStorage<'a, DeathBound>);

    fn run(&mut self, (entities, death_bound_store): Self::SystemData) {
        for (entity, death_bound) in (&entities, &death_bound_store).join() {
            if !entities.is_alive(death_bound.entity) {
                entities.delete(entity).unwrap();
            }
        }
    }
}
