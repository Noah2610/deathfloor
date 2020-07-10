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
        // TODO
        // if let Some(variant_name) = variant {
        //     if let Some(variant) = {
        //         entity_config
        //             .variants
        //             .as_ref()
        //             .and_then(|variants| variants.get(&variant_name).cloned())
        //     } {
        //         entity_config.merge(variant);
        //     }
        // }

        // TODO
        // COLLISION / SOLID TAGS
        // if let Some(collision_tag) = entity_config.collision_tag {
        //     let mut collider_storage =
        //         world.write_component::<Collider<CollisionTag>>();
        //     let mut collidable_storage =
        //         world.write_component::<Collidable<CollisionTag>>();
        //     collider_storage.insert(
        //         entity,
        //         Collider::new(CollisionTag::from(collision_tag.clone())),
        //     )?;
        //     collidable_storage.insert(
        //         entity,
        //         Collidable::new(CollisionTag::from(collision_tag)),
        //     )?;
        // }
        // if let Some(solid_tag) = entity_config.solid_tag {
        //     let mut solid_storage = world.write_component::<Solid<SolidTag>>();
        //     solid_storage
        //         .insert(entity, Solid::new(CollisionTag::from(solid_tag)))?;
        // }

        for (entity, entity_config_register) in
            (&entities, &mut entity_config_register_store).join()
        {
            for action in
                entity_config_register.drain_actions().collect::<Vec<_>>()
            {
                // use Act as EntityConfigRegisterAction;

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
    if let Some(mut variant) = entity_config_register
        .config
        .variants
        .as_ref()
        .and_then(|variants| variants.get(variant_name).cloned())
    {
        let variant_events = variant.events.take();
        let mut entity_config = variant;
        // NOTE: Only merge events, not components.
        //       Entity should already have all root components,
        //       unless they were overwritten by a variant.
        //       We should overwrite the root components with themselves,
        //       because this will remove any queued actions on any components.
        entity_config.events = entity_config_register.config.events.clone();
        entity_config.events.merge(variant_events);

        // EVENTS
        if let Some(events_register) = entity_config.events.clone() {
            events_register_store
                .insert(entity, events_register)
                .unwrap();
            // TODO: if the entity didn't have this already, then it should get it here.
            // action_type_trigger_storage
            //     .insert(entity, ActionTypeTrigger::default())?;
        }

        // COMPONENTS
        if let Some(components) = entity_config.components.clone() {
            insert_components(entity, components, components_stores).unwrap();
        }
    }
}
