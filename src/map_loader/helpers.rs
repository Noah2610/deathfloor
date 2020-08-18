pub(super) mod prelude {
    pub(in super::super) use super::base_entity;
    pub(in super::super) use super::edit_entity_with_entity_config;
    pub(in super::super) use super::get_sprite_render;
    pub use crate::animation_key::AnimationKey;
    pub use crate::components::prelude::*;
    pub use crate::helpers::resource;
    pub use crate::map_loader::map_data::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use amethyst::ecs::{Entity, EntityBuilder, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use deathframe::amethyst;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::resources::SpriteSheetHandles;
}

use crate::systems::system_helpers::prelude::insert_components;
use amethyst::ecs::WorldExt;
use deathframe::resources::SpriteSheetHandles;
use prelude::*;
use std::path::PathBuf;

pub(super) fn base_entity<'a, T>(
    world: &'a mut World,
    propful: &T,
) -> amethyst::Result<EntityBuilder<'a>>
where
    T: Propful,
{
    let mut transform: Transform = propful.pos().into();
    transform.set_translation_z(propful.z_or_default());

    let scale_x = propful
        .props()
        .get("scale_x")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    let scale_y = propful
        .props()
        .get("scale_y")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);

    let scale = transform.scale_mut();
    scale_x.map(|x| scale.x = x);
    scale_y.map(|y| scale.y = y);

    Ok(world
        .create_entity()
        .with(transform)
        .with(ScaleOnce::default())
        .with(Transparent))
}

pub(super) fn get_sprite_render<P>(
    world: &mut World,
    spritesheet_path: P,
    sprite_number: usize,
) -> amethyst::Result<SpriteRender>
where
    P: Into<PathBuf>,
{
    let handle = world
        .write_resource::<SpriteSheetHandles<PathBuf>>()
        .get_or_load(resource(spritesheet_path), world);
    Ok(SpriteRender {
        sprite_sheet:  handle,
        sprite_number: sprite_number,
    })
}

/// Edits `Entity` with the given `EntityConfig`.
pub(super) fn edit_entity_with_entity_config(
    world: &mut World,
    entity: Entity,
    mut entity_config: EntityConfig,
    variant_prop: Option<String>,
) -> amethyst::Result<()> {
    let variant = variant_prop.or(entity_config.default_variant.clone());

    // ENTITY_CONFIG_REGISTER
    // NOTE: Insert this first, so the inserted entity config
    //       is the one without the variant stuff merged.
    world
        .write_component::<EntityConfigRegister>()
        .insert(entity, EntityConfigRegister::new(entity_config.clone()))?;

    // Merge variant into entity config
    if let Some(variant_name) = variant {
        if let Some(variant) = {
            entity_config
                .variants
                .as_ref()
                .and_then(|variants| variants.get(&variant_name).cloned())
        } {
            entity_config.merge(variant);
        }
    }

    // COLLISION / SOLID TAGS
    if let Some(collision_tag) = entity_config.collision_tag {
        let mut collider_storage =
            world.write_component::<Collider<CollisionTag>>();
        let mut collidable_storage =
            world.write_component::<Collidable<CollisionTag>>();
        collider_storage.insert(
            entity,
            Collider::new(CollisionTag::from(collision_tag.clone())),
        )?;
        collidable_storage.insert(
            entity,
            Collidable::new(CollisionTag::from(collision_tag)),
        )?;
    }
    if let Some(solid_tag) = entity_config.solid_tag {
        let mut solid_storage = world.write_component::<Solid<SolidTag>>();
        solid_storage
            .insert(entity, Solid::new(CollisionTag::from(solid_tag)))?;
    }

    // EVENTS
    if let Some(events_register) = entity_config.events {
        let mut events_register_storage =
            world.write_component::<EventsRegister>();
        let mut action_type_trigger_storage =
            world.write_component::<ActionTypeTrigger>();
        events_register_storage.insert(entity, events_register)?;
        action_type_trigger_storage
            .insert(entity, ActionTypeTrigger::default())?;
    }

    // COMPONENTS
    if let Some(components) = entity_config.components {
        world.exec(|mut components_storages: EntityComponentsStorages| {
            insert_components(entity, components, &mut components_storages)
        })?;
    }

    Ok(())
}
