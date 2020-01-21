use super::system_prelude::*;

#[derive(Default)]
pub struct CheckGroundedSystem;

impl<'a> System<'a> for CheckGroundedSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, GeometricalWorldRes<f32>>,
        ColliderSet<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Grounded>,
    );

    fn run(
        &mut self,
        (
            entities,
            geometrical_world,
            collider_set,
            players,
            mut grounded_store,
        ): Self::SystemData,
    ) {
        // for (player_entity, player) in (&entities, &players).join() {
        //     if let Some(contact_events) = (*geometrical_world)
        //         .colliders_in_contact_with(&collider_set, player_entity)
        //     {
        //         for x in contact_events {
        //             // dbg!(&x);
        //         }
        //     }
        // }
    }
}
