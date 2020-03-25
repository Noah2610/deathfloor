use super::system_prelude::*;

#[derive(Default)]
pub struct HandleJumppadAffectedSystem;

impl<'a> System<'a> for HandleJumppadAffectedSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, JumppadAffected>,
        ReadStorage<'a, Jumppad>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, Movable>,
    );

    fn run(
        &mut self,
        (
            entities,
            jumppad_affected_store,
            jumppads,
            colliders,
            mut movables,
        ): Self::SystemData,
    ) {
        for (_, collider, movable) in
            (&jumppad_affected_store, &colliders, &mut movables).join()
        {
            let mut jumppad_strength_opt = None;

            'jumppads_loop: for (jumppad_entity, jumppad) in
                (&entities, &jumppads).join()
            {
                use deathframe::physics::query::exp::prelude_variants::*;

                if collider
                    .query::<FindQuery<CollisionTag>>()
                    .filter_ids(vec![jumppad_entity.id()])
                    .exp(&And(vec![
                        // TODO: This is unnecessary, and less flexible
                        // IsTag(CollisionTag::Jumppad),
                        IsState(Enter),
                        IsSide(Inner),
                    ]))
                    .run()
                    .is_some()
                {
                    jumppad_strength_opt = Some(jumppad.strength);
                    break 'jumppads_loop;
                }
            }

            if let Some(jumppad_strength) = jumppad_strength_opt {
                movable.add_action(MoveAction::SetVelocity {
                    velocity: jumppad_strength,
                });
            }
        }
    }
}
