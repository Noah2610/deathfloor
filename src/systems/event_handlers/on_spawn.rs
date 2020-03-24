use super::super::system_prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct HandleEventOnSpawn {
    triggered_entities: HashSet<Entity>,
}

impl<'a> System<'a> for HandleEventOnSpawn {
    type SystemData = (Entities<'a>, WriteStorage<'a, EventListener>);

    fn run(&mut self, (entities, mut event_listener_store): Self::SystemData) {
        let mut triggered_entities = HashSet::new();

        for (entity, event_listener_store) in
            (&entities, &mut event_listener_store).join()
        {
            if !self.triggered_entities.contains(&entity) {
                event_listener_store.trigger(&EventType::OnSpawn);
            }
            triggered_entities.insert(entity);
        }

        self.triggered_entities = triggered_entities;
    }
}
