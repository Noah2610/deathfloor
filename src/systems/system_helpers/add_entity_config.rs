use super::system_prelude::*;

/// Adds and applies an entity config to an entity.
/// Applies components, events, etc. from the entity config.
pub fn add_entity_config(
    entity: Entity,
    mut entity_config: EntityConfig,
    variant_prop: Option<String>,
    (
        settings,
        mut components_storages,
        mut entity_config_register_store,
        mut events_register_store,
        mut action_type_trigger_store,
    ): (
        ReadExpect<Settings>,
        EntityComponentsStorages,
        WriteStorage<EntityConfigRegister>,
        WriteStorage<EventsRegister>,
        WriteStorage<ActionTypeTrigger>,
    ),
) -> amethyst::Result<()> {
    // Inheritance is merged only once, here when loading the config.
    // The inheritance basically extends the entity config once.
    // The fully inheritance-merged entity config is set as the root entity config.
    // Inheritance is never touched again during runtime.
    //
    // Variant merges are different to inheritance merges.
    // Variants can be switched/merged here and during runtime,
    // but inheritance is only merged once here.
    //
    // ## ORDER OF OPERATIONS
    // 1)
    // Apply inheritance chain, recursively.
    // If the inherited entity config has another `inherits` field,
    // those configs are also merged.
    // Each variant, regardless of if it is active or not, should
    // merge its inheritance chain.
    //
    // 2)
    // After all inheritance chains have been merged,
    // we want to create the `EntityConfigRegister` with the root entity config.
    // This component is later inserted into the world.
    //
    // 3)
    // Now we apply the variant, by merging the variant with the root config.
    // Keep in mind that all inheritance has already been merged.
    //
    // 4)
    // After we've created our final entity config with the active variant,
    // and after we've told the `EntityConfigRegister` which variant is active (if any),
    // now we finally _insert_ the `EntityConfigRegister` into the world.
    //
    // 5)
    // Now we can insert all components and events
    // from the final, active entity config.

    // 1) APPLY INHERITANCE CHAIN
    entity_config = entity_config.with_inheritance(&settings.abstract_entities);

    // 2) CREATE ENTITY_CONFIG_REGISTER
    // NOTE: Insert this first, so the inserted entity config
    //       is the one without the variant stuff merged.
    //       Is created here and inserted a bit later.
    let mut entity_config_register =
        EntityConfigRegister::new(entity_config.clone());

    // 3) APPLY VARIANT
    let variant_name = variant_prop.or(entity_config.default_variant.clone());
    if let Some((merged_name, merged_variant)) =
        entity_config.merge_variant(variant_name)
    {
        entity_config_register.push_config(merged_name, merged_variant);
    }

    // 4) INSERT EntityConfigRegister COMPONENT
    entity_config_register_store.insert(entity, entity_config_register)?;

    // 5) INSERT COMPONENTS AND EVENTS

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
        insert_components(entity, components, &mut components_storages)?;
    }

    Ok(())
}
