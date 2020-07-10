use super::system_prelude::*;

#[derive(Default)]
pub struct BulletHitSystem;

impl<'a> System<'a> for BulletHitSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(&mut self, (entities, bullets, colliders): Self::SystemData) {
        for (entity, _, collider) in (&entities, &bullets, &colliders).join() {
            let in_tile_collision = {
                use deathframe::physics::query::exp::prelude_variants::*;

                collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&IsState(Enter))
                    .run()
                    .is_some()
            };

            if in_tile_collision {
                // REMOVE BULLET
                entities
                    .delete(entity)
                    .expect("Couldn't delete bullet entity");
            }
        }
    }
}
