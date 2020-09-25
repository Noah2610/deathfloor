use super::system_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Default)]
pub struct HandleEventDelay;

impl<'a> System<'a> for HandleEventDelay {
    type SystemData = (
        WriteStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            mut events_register_store,
            mut action_type_trigger_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, _) in (
            &mut events_register_store,
            &mut action_type_trigger_store,
            !&unloaded_store,
        )
            .join()
        {
            let delay_events: Vec<(u64, ActionType)> = events_register
                .events()
                .iter()
                .filter_map(|(event, action)| {
                    if let EventType::Delay(ms) = event {
                        Some((*ms, action.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            for (ms, action) in delay_events {
                let data_entry =
                    events_register.data.delay.entry(ms).or_insert_with(|| {
                        let mut timer = Timer::new(
                            Some(Duration::from_millis(ms).into()),
                            None,
                        );
                        timer.start().unwrap();
                        event_type_data::DelayData { timer }
                    });
                if data_entry.timer.state.is_running() {
                    data_entry.timer.update().unwrap();
                    if data_entry.timer.state.is_finished() {
                        action_type_trigger.add_action(action);
                    }
                }
            }
        }
    }
}
