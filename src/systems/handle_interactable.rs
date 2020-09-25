use super::system_prelude::*;

#[derive(Default)]
pub struct HandleInteractableSystem;

impl<'a> System<'a> for HandleInteractableSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        ReadStorage<'a, CanInteract>,
        WriteStorage<'a, Interactable>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            input_manager,
            can_interact_store,
            mut interactable_store,
            collider_store,
        ): Self::SystemData,
    ) {
        if input_manager.is_down(IngameActionBinding::Interact) {
            let collision_query = {
                use deathframe::physics::query::exp::prelude_variants::*;
                IsSide(Inner)
            };

            for (_, interactor_collider) in
                (&can_interact_store, &collider_store).join()
            {
                for (interactable_entity, interactable) in
                    (&entities, &mut interactable_store).join()
                {
                    if interactor_collider
                        .query::<FindQuery<CollisionTag>>()
                        .filter_ids(&vec![interactable_entity.id()])
                        .exp(&collision_query)
                        .run()
                        .is_some()
                    {
                        interactable.add_action(InteractableAction::Interacted);
                    }
                }
            }
        }
    }
}
