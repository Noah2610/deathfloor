use super::system_prelude::*;

#[derive(Default)]
pub struct HandleJumppadAffectedSystem;

impl<'a> System<'a> for HandleJumppadAffectedSystem {
    type SystemData = (
        ReadStorage<'a, JumppadAffected>,
        ReadStorage<'a, Jumppad>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, Movable>,
    );

    fn run(
        &mut self,
        (
            jumppad_affected_store,
            jumppads,
            colliders,
            mut movables,
        ): Self::SystemData,
    ) {
        for (_, collider, movable) in
            (&jumppad_affected_store, &colliders, &mut movables).join()
        {
            let is_in_jumppad_collision = {
                // use deathframe::physics::query::exp::variant_prelude::*;

                // let matches =
                //     collider.query::<(), ()>().find((), IsState(Enter)).run();
                false
            };
        }
    }
}
