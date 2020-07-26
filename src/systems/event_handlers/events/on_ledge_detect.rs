use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnLedgeDetectSystem;

impl<'a> System<'a> for HandleEventOnLedgeDetectSystem {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        WriteStorage<'a, LedgeDetector>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            mut ledge_detector_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, ledge_detector) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &mut ledge_detector_store,
        )
            .join()
        {
            for LedgeDetectorDetected {
                corner,
                if_touching,
            } in ledge_detector.drain_detected()
            {
                let event_type = EventType::OnLedgeDetect(corner, if_touching);
                if let Some(action) =
                    events_register.get_action(&event_type).cloned()
                {
                    action_type_trigger.add_action(action);
                }
            }
        }
    }
}
