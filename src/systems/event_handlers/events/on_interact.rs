use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnInteract;

impl<'a> System<'a> for HandleEventOnInteract {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        WriteStorage<'a, Interactable>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            mut interactable_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (
            events_register,
            action_type_trigger,
            interactable,
            loadable_opt,
            loaded_opt,
        ) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &mut interactable_store,
            loadable_store.maybe(),
            loaded_store.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
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
}
