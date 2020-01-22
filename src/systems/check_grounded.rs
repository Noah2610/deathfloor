use super::system_prelude::*;

use specs_physics::nalgebra::geometry::{Isometry2, Point2};
use specs_physics::ncollide::query::{Ray, RayCast};

#[derive(Default)]
pub struct CheckGroundedSystem;

impl<'a> System<'a> for CheckGroundedSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, GeometricalWorldRes<f32>>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, ColliderComponent>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Grounded>,
    );

    fn run(
        &mut self,
        (
            entities,
            geometrical_world,
            transforms,
            colliders,
            players,
            mut grounded_store,
        ): Self::SystemData,
    ) {
        for (player_entity, player, player_transform, player_collider) in
            (&entities, &players, &transforms, &colliders).join()
        {
            // eprintln!("PLAYER: {}", player_entity.id());

            // let pos = player_transform.translation();
            // let point = Point2::new(pos.x, pos.y);
            // let shape = player_collider.0.shape();
            // let ray = Ray::new(point, Vector::new(0.0, -10.0));

            // if shape.intersects_ray(&Isometry2::identity(), &ray) {
            //     dbg!("OMG IT COLLIDE!");
            // } else {
            //     dbg!("nah");
            // }

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
