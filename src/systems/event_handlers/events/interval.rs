use super::system_prelude::*;
use climer::Timer;
use std::collections::{HashMap, HashSet};
use std::time::Duration;

#[derive(Default)]
pub struct HandleEventInterval {
    registered:       HashMap<Entity, (Timer, ActionType)>,
    ignored_entities: HashSet<Entity>,
}

impl<'a> System<'a> for HandleEventInterval {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
    );

    fn run(
        &mut self,
        (
            entities,
            events_register_store,
            mut action_type_trigger_store,
        ): Self::SystemData,
    ) {
        for (entity, events_register, action_type_trigger) in (
            &entities,
            &events_register_store,
            &mut action_type_trigger_store,
        )
            .join()
        {
            if !self.ignored_entities.contains(&entity) {
                if let Some((timer, action)) = self.registered.get_mut(&entity)
                {
                    // Update timer
                    timer.update().unwrap();
                    if timer.state.is_finished() {
                        action_type_trigger.add_action(action.clone());
                        timer.start().unwrap();
                    }
                } else {
                    // Register timer
                    let mut has_interval_event = false;
                    for (event, action) in events_register.events() {
                        if let EventType::Interval(delay_ms) = event {
                            has_interval_event = true;
                            let mut timer = Timer::new(
                                Some(Duration::from_millis(*delay_ms).into()),
                                None,
                            );
                            timer.start().unwrap();
                            self.registered
                                .insert(entity, (timer, action.clone()));
                        }
                    }
                    if !has_interval_event {
                        self.ignored_entities.insert(entity);
                    }
                }
            }
        }
    }
}
