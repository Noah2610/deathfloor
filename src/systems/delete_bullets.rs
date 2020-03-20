use super::system_prelude::*;

#[derive(Default)]
pub struct DeleteBulletsSystem;

impl<'a> System<'a> for DeleteBulletsSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Bullet>);

    fn run(&mut self, (entities, mut bullets): Self::SystemData) {
        for (entity, bullet) in (&entities, &mut bullets).join() {
            if bullet.despawn_timer.state.is_stopped() {
                bullet.despawn_timer.start().unwrap();
            }
            bullet.despawn_timer.update().unwrap();
            if bullet.despawn_timer.state.is_finished() {
                entities
                    .delete(entity)
                    .expect("Couldn't delete bullet entity");
            }
        }
    }
}
