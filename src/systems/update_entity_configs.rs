use super::system_prelude::*;
use crate::entity_config::prelude::*;

#[derive(Default)]
pub struct UpdateEntityConfigsSystem;

impl<'a> System<'a> for UpdateEntityConfigsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, EntityConfigRegister>,
        WriteStorage<'a, EventsRegister>,
        EntityConfigComponentsStorages<'a>,
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
                    EntityConfigRegisterAction::PushVariant(variant_name) => {
                        push_variant(
                            entity,
                            entity_config_register,
                            &variant_name,
                            &mut events_register_store,
                            &mut components_stores,
                        )
                    }
                    EntityConfigRegisterAction::PopVariant => pop_variant(
                        entity,
                        entity_config_register,
                        &mut events_register_store,
                        &mut components_stores,
                    ),
                    EntityConfigRegisterAction::ApplyComponents => {
                        apply_components(
                            entity,
                            entity_config_register,
                            &mut components_stores,
                        )
                    }
                }
            }
        }
    }
}

fn get_variant_from_register(
    register: &EntityConfigRegister,
    target_variant: &str,
) -> Option<EntityConfig> {
    let variant = register.get_variant(target_variant);
    if variant.is_none() {
        eprintln!(
            "[WARNING]\n    Tried to switch or push a variant that doesn't \
             exist: {}",
            target_variant
        );
    }
    variant
}

fn switch_variant(
    entity: Entity,
    entity_config_register: &mut EntityConfigRegister,
    target_variant: &str,
    events_register_store: &mut WriteStorage<EventsRegister>,
    components_stores: &mut EntityConfigComponentsStorages,
) {
    if let Some(variant) =
        get_variant_from_register(entity_config_register, target_variant)
    {
        entity_config_register
            .switch_config(target_variant.to_string(), variant.clone());
        let mut entity_config = entity_config_register.root_config.clone();
        // NOTE: Ignore root entity config components.
        //       These components should already have been inserted.
        //       If we would re-insert them, then all queued actions
        //       for the components would be removed, which we don't want.
        entity_config.components = None;
        entity_config.merge(variant);

        apply_entity_config(
            entity,
            entity_config,
            events_register_store,
            components_stores,
        );
    }
}

fn push_variant(
    entity: Entity,
    entity_config_register: &mut EntityConfigRegister,
    target_variant: &str,
    events_register_store: &mut WriteStorage<EventsRegister>,
    components_stores: &mut EntityConfigComponentsStorages,
) {
    if let Some(variant) =
        get_variant_from_register(entity_config_register, target_variant)
    {
        entity_config_register
            .push_config(target_variant.to_string(), variant.clone());
        let mut entity_config = entity_config_register.root_config.clone();
        entity_config.components = None;
        entity_config.merge(variant);

        apply_entity_config(
            entity,
            entity_config,
            events_register_store,
            components_stores,
        );
    }
}

fn pop_variant(
    entity: Entity,
    entity_config_register: &mut EntityConfigRegister,
    events_register_store: &mut WriteStorage<EventsRegister>,
    components_stores: &mut EntityConfigComponentsStorages,
) {
    let mut entity_config = entity_config_register.root_config.clone();
    entity_config.components = None;
    if let None = entity_config_register.pop_config() {
        eprintln!(
            "[WARNING]\n   Attempted to pop off variant from entity config \
             stack,\n    but the stack is empty."
        );
    }
    if let Some((_, variant_config)) =
        entity_config_register.config_stack.last().cloned()
    {
        entity_config.merge(variant_config);
    }

    apply_entity_config(
        entity,
        entity_config,
        events_register_store,
        components_stores,
    );
}

fn apply_entity_config(
    entity: Entity,
    entity_config: EntityConfig,
    events_register_store: &mut WriteStorage<EventsRegister>,
    components_stores: &mut EntityConfigComponentsStorages,
) {
    // EVENTS
    if let Some(events_register) = entity_config.events {
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
        if let Some(existing_solid) = components_stores.solid.get_mut(entity) {
            existing_solid.tag = solid_tag;
        } else {
            components_stores
                .solid
                .insert(entity, Solid::new(solid_tag))
                .unwrap();
        }
    }

    // COMPONENTS
    // Insert `Size` component first.
    // TODO: why this no work?
    if let Some(size) = entity_config.size {
        components_stores.size.insert(entity, size).unwrap();
    }
    // Insert all other components.
    if let Some(components) = entity_config.components {
        insert_components(entity, components, components_stores).unwrap();
    }
}

fn apply_components(
    entity: Entity,
    entity_config_register: &mut EntityConfigRegister,
    components_stores: &mut EntityConfigComponentsStorages,
) {
    let components = entity_config_register
        .config_stack
        .last()
        .map(|(_, config)| config)
        .unwrap_or(&entity_config_register.root_config)
        .components
        .clone();
    if let Some(components) = components {
        insert_components(entity, components, components_stores).unwrap();
    }
}
