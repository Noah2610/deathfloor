use super::system_prelude::*;
use climer::Timer;
use std::time::Duration;

#[derive(Default)]
pub struct HandleEventInterval;

impl<'a> System<'a> for HandleEventInterval {
    type SystemData = (
        WriteStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            mut events_register_store,
            mut action_type_trigger_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (events_register, action_type_trigger, loadable_opt, loaded_opt) in
            (
                &mut events_register_store,
                &mut action_type_trigger_store,
                loadable_store.maybe(),
                loaded_store.maybe(),
            )
                .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                let interval_events: Vec<(u64, ActionType)> = events_register
                    .events()
                    .iter()
                    .filter_map(|(event, action)| {
                        if let EventType::Interval(ms) = event {
                            Some((*ms, action.clone()))
                        } else {
                            None
                        }
                    })
                    .collect();

                for (ms, action) in interval_events {
                    let data_entry =
                        events_register.data.interval.entry(ms).or_insert_with(
                            || {
                                let mut timer = Timer::new(
                                    Some(Duration::from_millis(ms).into()),
                                    None,
                                );
                                timer.start().unwrap();
                                event_type_data::IntervalData { timer }
                            },
                        );
                    data_entry.timer.update().unwrap();
                    if data_entry.timer.state.is_finished() {
                        action_type_trigger.add_action(action);
                        data_entry.timer.start().unwrap();
                    }
                }
            }
        }
    }
}
