use super::helpers::prelude::*;

/// Builds the custom entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
    custom_type: String,
) -> amethyst::Result<Entity> {
    let settings = world
        .read_resource::<Settings>()
        .custom_entities
        .types
        .get(&custom_type)
        .cloned()
        .ok_or_else(|| {
            amethyst::Error::from_string(format!(
                "No settings for custom entity {}",
                custom_type
            ))
        })?;

    let sprite_render_opt = settings.spritesheet_filename.map(|filename| {
        get_sprite_render(world, format!("spritesheets/{}", filename), 0)
    });

    let mut entity_builder =
        base_object_entity(world, object)?.with::<Size>(object.size.into());

    if let Some(sprite_render) = sprite_render_opt {
        entity_builder = entity_builder.with(sprite_render?);
    }

    let entity = entity_builder.build();

    if let Some(entity_config) = settings.entity {
        edit_entity_with_entity_config(
            world,
            entity,
            entity_config,
            object.variant(),
        )?;
    }

    Ok(entity)
}
