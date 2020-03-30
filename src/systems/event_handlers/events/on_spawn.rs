use super::system_prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct HandleEventOnSpawn {
    triggered_entities: HashSet<Entity>,
}

impl<'a> System<'a> for HandleEventOnSpawn {
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
        let mut triggered_entities = HashSet::new();

        for (entity, events_register, action_type_trigger) in (
            &entities,
            &events_register_store,
            &mut action_type_trigger_store,
        )
            .join()
        {
            if !self.triggered_entities.contains(&entity) {
                if let Some(action) =
                    events_register.get_action(&EventType::OnSpawn).cloned()
                {
                    action_type_trigger.trigger(action);
                }
            }
            triggered_entities.insert(entity);
        }

        self.triggered_entities = triggered_entities;
    }
}
