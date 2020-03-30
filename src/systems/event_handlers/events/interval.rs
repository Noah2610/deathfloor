use super::system_prelude::*;
use climer::Timer;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct HandleEventInterval {
    timers:           HashMap<Entity, Timer>,
    ignored_entities: HashSet<Entity>,
}

impl<'a> System<'a> for HandleEventInterval {
    type SystemData = (Entities<'a>, WriteStorage<'a, EventListener>);

    fn run(&mut self, (entities, mut event_listener_store): Self::SystemData) {
        for (entity, event_listener) in
            (&entities, &mut event_listener_store).join()
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
