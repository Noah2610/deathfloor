use super::system_prelude::*;
use deathframe::physics::query::prelude::FindQuery;

#[derive(Default)]
pub struct HandleDeathOnContactSystem;

impl<'a> System<'a> for HandleDeathOnContactSystem {
    type SystemData = (
        ReadStorage<'a, DeathOnContact>,
        WriteStorage<'a, Lifecycle>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            death_on_contact_store,
            mut lifecycle_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (death_on_contact, lifecycle, collider) in (
            &death_on_contact_store,
            &mut lifecycle_store,
            &collider_store,
        )
            .join()
        {
            if let LifecycleState::Alive = lifecycle.state {
                let query = {
                    use deathframe::physics::query::exp::prelude_variants::*;
                    let collision_tag = CollisionTag {
                        labels:        Vec::new(),
                        collides_with: death_on_contact.collides_with.clone(),
                    };
                    And(vec![IsState(Enter), IsTag(collision_tag)])
                };
                let in_contact = collider
                    .query::<FindQuery<CollisionTag>>()
                    .exp(&query)
                    .run()
                    .is_some();
                if in_contact {}
            }
        }
    }
}
