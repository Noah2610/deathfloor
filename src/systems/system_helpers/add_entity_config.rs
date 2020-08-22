use super::system_prelude::*;

/// Adds and applies an entity config to an entity.
/// Applies components, events, etc. from the entity config.
pub fn add_entity_config(
    entity: Entity,
    mut entity_config: EntityConfig,
    variant_prop: Option<String>,
    components_storages: &mut EntityComponentsStorages,
    entity_config_register_store: &mut WriteStorage<EntityConfigRegister>,
    events_register_store: &mut WriteStorage<EventsRegister>,
    action_type_trigger_store: &mut WriteStorage<ActionTypeTrigger>,
) -> amethyst::Result<()> {
    let variant = variant_prop.or(entity_config.default_variant.clone());

    // ENTITY_CONFIG_REGISTER
    // NOTE: Insert this first, so the inserted entity config
    //       is the one without the variant stuff merged.
    //       Is created here and inserted a bit later.
    let mut entity_config_register =
        EntityConfigRegister::new(entity_config.clone());

    // Merge variant into entity config
    if let Some(variant_name) = variant {
        if let Some(variant) = {
            entity_config
                .variants
                .as_ref()
                .and_then(|variants| variants.get(&variant_name).cloned())
        } {
            entity_config_register.push_config(variant.clone());
            entity_config.merge(variant);
        }
    }

    entity_config_register_store.insert(entity, entity_config_register)?;

    // COLLISION / SOLID TAGS
    if let Some(collision_tag) = entity_config.collision_tag {
        components_storages.collider.insert(
            entity,
            Collider::new(CollisionTag::from(collision_tag.clone())),
        )?;
        components_storages.collidable.insert(
            entity,
            Collidable::new(CollisionTag::from(collision_tag)),
        )?;
    }
    if let Some(solid_tag) = entity_config.solid_tag {
        components_storages
            .solid
            .insert(entity, Solid::new(CollisionTag::from(solid_tag)))?;
    }

    // EVENTS
    if let Some(events_register) = entity_config.events {
        events_register_store.insert(entity, events_register)?;
        action_type_trigger_store
            .insert(entity, ActionTypeTrigger::default())?;
    }

    // COMPONENTS
    if let Some(components) = entity_config.components {
        insert_components(entity, components, components_storages)?;
    }

    Ok(())
}
