use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnLedgeDetectSystem;

impl<'a> System<'a> for HandleEventOnLedgeDetectSystem {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        WriteStorage<'a, LedgeDetector>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            mut ledge_detector_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (
            events_register,
            action_type_trigger,
            ledge_detector,
            loadable_opt,
            loaded_opt,
        ) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &mut ledge_detector_store,
            loadable_store.maybe(),
            loaded_store.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                for LedgeDetectorDetected {
                    corner,
                    if_touching,
                } in ledge_detector.drain_detected()
                {
                    let event_type =
                        EventType::OnLedgeDetect(corner, if_touching);
                    if let Some(action) =
                        events_register.get_action(&event_type).cloned()
                    {
                        action_type_trigger.add_action(action);
                    }
                }
            }
        }
    }
}
