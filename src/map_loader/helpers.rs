pub(super) mod prelude {
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
    variant: Option<String>,
) -> amethyst::Result<()> {
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

    // ENTITY_CONFIG_REGISTER
    world
        .write_component::<EntityConfigRegister>()
        .insert(entity, EntityConfigRegister::new(entity_config.clone()))?;

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
