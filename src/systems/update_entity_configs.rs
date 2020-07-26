use super::system_prelude::*;

#[derive(Default)]
pub struct UpdateEntityConfigsSystem;

impl<'a> System<'a> for UpdateEntityConfigsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, EntityConfigRegister>,
        WriteStorage<'a, EventsRegister>,
        EntityComponentsStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut entity_config_register_store,
            mut events_register_store,
            mut components_stores,
        ): Self::SystemData,
    ) {
        for (entity, entity_config_register) in
            (&entities, &mut entity_config_register_store).join()
        {
            for action in
                entity_config_register.drain_actions().collect::<Vec<_>>()
            {
                match action {
                    EntityConfigRegisterAction::SwitchVariant(variant_name) => {
                        switch_variant(
                            entity,
                            entity_config_register,
                            &variant_name,
                            &mut events_register_store,
                            &mut components_stores,
                        )
                    }
                }
            }
        }
    }
}

fn switch_variant(
    entity: Entity,
    entity_config_register: &mut EntityConfigRegister,
    variant_name: &str,
    events_register_store: &mut WriteStorage<EventsRegister>,
    components_stores: &mut EntityComponentsStorages,
) {
    if let Some(variant) = entity_config_register
        .config
        .variants
        .as_ref()
        .and_then(|variants| variants.get(variant_name).cloned())
    {
        let mut entity_config = entity_config_register.config.clone();
        // NOTE: Ignore root entity config components.
        //       These components should already have been inserted.
        //       If we would re-insert them, then all queued actions
        //       for the components would be removed, which we don't want.
        entity_config.components = None;
        entity_config.merge(variant);

        // EVENTS
        if let Some(events_register) = entity_config.events.clone() {
            events_register_store
                .insert(entity, events_register)
                .unwrap();
            // TODO: if the entity didn't have this already, then it should get it here.
            // action_type_trigger_storage
            //     .insert(entity, ActionTypeTrigger::default())?;
        }

        // COLLISION / SOLID TAGS
        if let Some(collision_tag) = entity_config.collision_tag {
            components_stores
                .collider
                .insert(
                    entity,
                    Collider::new(CollisionTag::from(collision_tag.clone())),
                )
                .unwrap();
            components_stores
                .collidable
                .insert(
                    entity,
                    Collidable::new(CollisionTag::from(collision_tag)),
                )
                .unwrap();
        }
        if let Some(solid_tag) = entity_config.solid_tag {
            components_stores
                .solid
                .insert(entity, Solid::new(CollisionTag::from(solid_tag)))
                .unwrap();
        }

        // COMPONENTS
        if let Some(components) = entity_config.components.clone() {
            insert_components(entity, components, components_stores).unwrap();
        }
    }
}
