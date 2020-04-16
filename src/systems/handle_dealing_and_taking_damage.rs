use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleDealingAndTakingDamageSystem;

impl<'a> System<'a> for HandleDealingAndTakingDamageSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, DealsDamage>,
        ReadStorage<'a, TakesDamage>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, HealthActionQueue>,
    );

    fn run(
        &mut self,
        (
            entities,
            deals_damage_store,
            takes_damage_store,
            collider_store,
            mut health_action_queue_store,
        ): Self::SystemData,
    ) {
        let mut damage_map = HashMap::new();

        for (entity, deals_damage) in (&entities, &deals_damage_store).join() {
            damage_map.insert(entity.id(), deals_damage.damage);
        }

        let damage_dealing_ids: Vec<Index> =
            damage_map.keys().cloned().collect();

        for (_entity, takes_damage, collider, health_action_queue) in (
            &entities,
            &takes_damage_store,
            &collider_store,
            &mut health_action_queue_store,
        )
            .join()
        {
            let takes_damage_from_tag =
                CollisionTag::from(takes_damage.takes_damage_from.clone());
            let query_exp = {
                use deathframe::physics::query::exp::prelude_variants::*;
                And(vec![IsTag(takes_damage_from_tag), IsState(Enter)])
            };

            let collisions = collider
                .query::<FilterQuery<CollisionTag>>()
                .filter_ids(damage_dealing_ids.clone())
                .exp(&query_exp)
                .run();

            for collision in collisions {
                if let Some(damage) = damage_map.get(&collision.id) {
                    health_action_queue.lose(*damage);
                }
            }
        }
    }
}
