use super::system_prelude::*;
use deathframe::physics::query::prelude::FindQuery;

#[derive(Default)]
pub struct HandleDeathOnContactSystem;

impl<'a> System<'a> for HandleDeathOnContactSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DeathOnContact>,
        WriteStorage<'a, Lifecycle>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            death_on_contact_store,
            mut lifecycle_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (entity, death_on_contact, lifecycle_opt, collider) in (
            &entities,
            &death_on_contact_store,
            (&mut lifecycle_store).maybe(),
            &collider_store,
        )
            .join()
        {
            if let Some(LifecycleState::Alive) | None =
                lifecycle_opt.as_ref().map(|lifecycle| &lifecycle.state)
            {
                let query = {
                    use deathframe::physics::query::exp::prelude_variants::*;
                    let collision_tag = CollisionTag {
                        labels:        death_on_contact.collides_with.clone(),
                        collides_with: Vec::new(),
                    };
                    And(vec![IsState(Enter), IsTag(collision_tag)])
                };
                let in_contact = collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&query)
                    .run()
                    .is_some();
                if in_contact {
                    if let Some(lifecycle) = lifecycle_opt {
                        lifecycle.state = LifecycleState::Death;
                    } else {
                        entities.delete(entity).unwrap();
                    }
                }
            }
        }
    }
}
