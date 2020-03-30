use super::system_prelude::*;
use climer::Timer;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct HandleEventInterval {
    timers:           HashMap<Entity, Timer>,
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
            // TODO
            // if !self.ignored_entities.contains(&entity) {
            //     if let Some(timer) = self.timers.get_mut() {
            //     } else {
            //         let event_types = event_listener_store.events();
            //     }
            // }
        }
    }
}
