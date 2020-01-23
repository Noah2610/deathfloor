use super::system_prelude::*;

use specs_physics::nalgebra::geometry::{Isometry2, Point2};
use specs_physics::ncollide::query::{Ray, RayCast};

#[derive(Default)]
pub struct CheckGroundedSystem;

impl<'a> System<'a> for CheckGroundedSystem {
    type SystemData = (
        ReadStorage<'a, Transform>,
        ReadStorage<'a, ColliderComponent>,
        ReadStorage<'a, Solid>,
        WriteStorage<'a, Grounded>,
    );

    fn run(
        &mut self,
        (transforms, colliders, solids, mut grounded_store): Self::SystemData,
    ) {
        for (grounded, grounded_transform, grounded_collider) in
            (&mut grounded_store, &transforms, &colliders).join()
        {
            let ray_origin = {
                let pos = grounded_transform.translation();
                let player_shape = grounded_collider.shape();
                let aabb = player_shape
                    .aabb(&Isometry2::new(Vector::new(pos.x, pos.y), 0.0));
                let mut point = aabb.center();
                point.y -= aabb.half_extents().y;
                point
            };
            let ray = Ray::<f32>::new(ray_origin, Vector::y() * -1.0);
            let mut has_ray_hit = false;

            for (_, solid_transform, solid_collider) in
                (&solids, &transforms, &colliders).join()
            {
                let pos = {
                    let translation = solid_transform.translation();
                    Vector::new(translation.x, translation.y)
                };
                let isometry = Isometry2::new(pos, 0.0);
                let shape = solid_collider.0.shape();
                if let Some(toi) = shape.toi_with_ray(&isometry, &ray, true) {
                    if toi <= grounded.padding {
                        has_ray_hit = true;
                        break;
                    }
                }
            }

            grounded.set_on_ground(has_ray_hit);
        }
    }
}
