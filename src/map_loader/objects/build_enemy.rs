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
        format!("spritesheets/{}", &enemy_settings.spritesheet_filename),
        0,
    )?;

    let entity = base_object_entity(world, object)?
        .with::<Size>(object.size.into())
        .with(Enemy::new(enemy_type))
        .with(Loadable::default())
        .with(sprite_render)
        .with(Hidden)
        .with(Velocity::default())
        .with(Movable::default())
        .with(SoundPlayer::<SoundType>::default())
        .build();

    let variant = object.variant();

    edit_entity_with_entity_config(
        world,
        entity,
        enemy_settings.entity,
        variant,
    )?;

    Ok(entity)
}
