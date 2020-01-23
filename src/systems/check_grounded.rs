use super::system_prelude::*;

use specs_physics::nalgebra::geometry::{Isometry2, Point2};
use specs_physics::ncollide::query::{Ray, RayCast};

#[derive(Default)]
pub struct CheckGroundedSystem;

impl<'a> System<'a> for CheckGroundedSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Tile>,
        ReadStorage<'a, ColliderComponent>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Grounded>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            tiles,
            colliders,
            players,
            mut grounded_store,
        ): Self::SystemData,
    ) {
        for (player_entity, player, player_transform, player_collider) in
            (&entities, &players, &transforms, &colliders).join()
        {
            eprintln!("PLAYER: {}", player_entity.id());

            let ray_origin = {
                let pos = player_transform.translation();
                let player_shape = player_collider.shape();
                let aabb = player_shape
                    .aabb(&Isometry2::new(Vector::new(pos.x, pos.y), 0.0));
                let mut point = aabb.center();
                point.y -= aabb.half_extents().y;
                point
            };
            // let shape = player_collider.0.shape();
            // let ray = Ray::new(point, Vector::new(0.0, -10.0));

            let ray = Ray::<f32>::new(ray_origin, Vector::y() * -1.0);
            // dbg!(&ray);

            let mut ray_toi = None;

            for (_, tile_transform, tile_collider) in
                (&tiles, &transforms, &colliders).join()
            {
                let pos = {
                    let translation = tile_transform.translation();
                    Vector::new(translation.x, translation.y)
                };
                let isometry = Isometry2::new(pos, 0.0);
                let shape = tile_collider.0.shape();

                if let Some(toi) = shape.toi_with_ray(&isometry, &ray, true) {
                    ray_toi = Some(toi);
                    break;
                }
            }

            if let Some(toi) = ray_toi {
                dbg!(toi);
            } else {
                dbg!("nah");
            }

            // let ray = Ray::new();
            // let player_shape = player_collider.0.shape();
            // player_shape.intersects_ray();

            // if let Some(contact_events) = (*geometrical_world)
            //     .colliders_in_contact_with(&collider_set, player_entity)
            // {
            //     for (contact_entity, contact_collider) in contact_events {
            //         eprintln!("CONTACT: {}", contact_entity.id());
            //     }
            // }
        }
    }
}
