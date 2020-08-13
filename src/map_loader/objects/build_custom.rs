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

    let size = settings
        .entity
        .components
        .as_ref()
        .and_then(|comps| comps.size.clone())
        .unwrap_or(object.size.into());

    let sprite_render_opt =
        settings.spritesheet_filename.as_ref().map(|filename| {
            get_sprite_render(world, format!("spritesheets/{}", filename), 0)
        });

    let mut entity_builder = base_object_entity(world, object)?.with(size);

    if let Some(sprite_render) = sprite_render_opt {
        entity_builder = entity_builder.with(sprite_render?);
    }

    let entity = entity_builder.build();

    let variant = object
        .props
        .get("variant")
        .and_then(|val| val.as_str())
        .map(ToString::to_string);

    edit_entity_with_entity_config(world, entity, settings.entity, variant)?;

    Ok(entity)
}
