use super::helpers::prelude::*;

/// Builds the enemy entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
    enemy_type: EnemyType,
) -> amethyst::Result<Entity> {
    let enemy_settings = world
        .read_resource::<Settings>()
        .enemies
        .types
        .get(&enemy_type)
        .cloned()
        .ok_or_else(|| {
            amethyst::Error::from_string(format!(
                "No settings for EnemyType::{}",
                enemy_type
            ))
        })?;

    let sprite_render = get_sprite_render(
        world,
        format!("spritesheets/{}", enemy_settings.spritesheet_filename),
        1,
    )?;

    // CREATE ENTITY_BUILDER

    let mut entity_builder = base_object_entity(world, object)?
        .with(Loadable::default())
        .with(Hidden)
        .with(sprite_render);

    // CONFIGURABLE COMPONENTS

    if let Some(size) = enemy_settings.components.size {
        entity_builder = entity_builder.with(size);
    }
    if let Some(gravity) = enemy_settings.components.gravity {
        entity_builder = entity_builder.with(gravity);
    }

    Ok(entity_builder.build())
}
