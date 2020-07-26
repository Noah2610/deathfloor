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
        // Update existing collision / solid tags if entity already has Collider / Solid,
        // or insert new Collider / Solid if it doesn't.
        if let Some(collision_tag_wrapper) = entity_config.collision_tag {
            let collision_tag: CollisionTag = collision_tag_wrapper.into();

            if let Some(existing_collider) =
                components_stores.collider.get_mut(entity)
            {
                existing_collider.tag = collision_tag.clone();
            } else {
                components_stores
                    .collider
                    .insert(entity, Collider::new(collision_tag.clone()))
                    .unwrap();
            }
            if let Some(existing_collidable) =
                components_stores.collidable.get_mut(entity)
            {
                existing_collidable.tag = collision_tag;
            } else {
                components_stores
                    .collidable
                    .insert(entity, Collidable::new(collision_tag))
                    .unwrap();
            }
        }
        if let Some(solid_tag_wrapper) = entity_config.solid_tag {
            let solid_tag: SolidTag = solid_tag_wrapper.into();
            if let Some(existing_solid) =
                components_stores.solid.get_mut(entity)
            {
                existing_solid.tag = solid_tag;
            } else {
                components_stores
                    .solid
                    .insert(entity, Solid::new(solid_tag))
                    .unwrap();
            }
        }

        // COMPONENTS
        if let Some(components) = entity_config.components.clone() {
            insert_components(entity, components, components_stores).unwrap();
        }
    }
}
