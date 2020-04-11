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

    let size = enemy_settings
        .entity
        .components
        .as_ref()
        .and_then(|comps| comps.size.clone())
        .unwrap_or(object.size.into());

    let sprite_render = get_sprite_render(
        world,
        format!("spritesheets/{}", &enemy_settings.spritesheet_filename),
        1, // TODO
    )?;

    let entity = base_object_entity(world, object)?
        .with(size)
        .with(Enemy::new(enemy_type))
        .with(Loadable::default())
        .with(sprite_render)
        .with(Hidden)
        .with(Velocity::default())
        .with(Movable::default())
        .with(SoundPlayer::<SoundType>::default())
        .build();

    edit_entity_with_entity_config(world, entity, enemy_settings.entity)?;

    Ok(entity)
}
