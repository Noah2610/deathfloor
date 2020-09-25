use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnInteract;

impl<'a> System<'a> for HandleEventOnInteract {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        WriteStorage<'a, Interactable>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            mut interactable_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, interactable, _) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &mut interactable_store,
            !&unloaded_store,
        )
            .join()
        {
            if let Some(action) =
                events_register.get_action(&EventType::OnInteract)
            {
                for interactable_action in interactable.drain_actions() {
                    match interactable_action {
                        InteractableAction::Interacted => {
                            action_type_trigger.add_action(action.clone());
                        }
                    }
                }
            }
        }
    }
}
