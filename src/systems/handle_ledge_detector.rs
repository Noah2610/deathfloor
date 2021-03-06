use super::system_prelude::*;
use deathframe::physics::query::prelude::FindQuery;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleLedgeDetectorSystem;

impl<'a> System<'a> for HandleLedgeDetectorSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, LedgeDetector>,
        ReadStorage<'a, LedgeDetectorCornerDetector>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut ledge_detector_store,
            corner_detector_store,
            collider_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let mut corner_collisions = HashMap::<
            Entity,
            Vec<(LedgeDetectorCorner, LedgeDetectorSide)>,
        >::new();

        for (corner_detector, corner_collider) in
            (&corner_detector_store, &collider_store).join()
        {
            if corner_collider.collisions.is_empty() {
                let entry = corner_collisions
                    .entry(corner_detector.owner)
                    .or_insert(Default::default());
                entry.push((
                    corner_detector.corner,
                    corner_detector.if_touching,
                ));
            }
        }

        for (entity, ledge_detector, collider, _) in (
            &entities,
            &mut ledge_detector_store,
            &collider_store,
            !&unloaded_store,
        )
            .join()
        {
            if let Some(collisions) = corner_collisions.get(&entity) {
                for (corner, side) in collisions {
                    let query_exp = {
                        use deathframe::physics::query::exp::prelude_variants::*;
                        IsSide(side.into())
                    };

                    if collider
                        .query::<FindQuery<CollisionTag>>()
                        .exp(&query_exp)
                        .run()
                        .is_some()
                    {
                        ledge_detector.add_detected(LedgeDetectorDetected {
                            corner:      *corner,
                            if_touching: *side,
                        });
                    }
                }
            }
        }
    }
}
