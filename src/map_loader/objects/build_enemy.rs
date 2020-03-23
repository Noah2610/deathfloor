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

    let size = enemy_settings.components.size.unwrap_or(object.size.into());

    let sprite_render = get_sprite_render(
        world,
        format!("spritesheets/{}", enemy_settings.spritesheet_filename),
        1,
    )?;

    // CREATE ENTITY_BUILDER

    let mut entity_builder = base_object_entity(world, object)?
        .with(Loadable::default())
        .with(Hidden)
        .with(size.clone())
        .with(sprite_render)
        .with(Velocity::default());

    // CONFIGURABLE COMPONENTS

    if let Some(gravity) = enemy_settings.components.gravity {
        entity_builder = entity_builder.with(gravity);
    }
    if let Some(animations) = enemy_settings.components.animations {
        entity_builder = entity_builder.with(animations);
    }
    if let Some(hitbox_config) = enemy_settings.components.hitbox {
        let hitbox = match hitbox_config {
            HitboxConfig::Size => Hitbox::new().with_rect(Rect::from(&size)),
            HitboxConfig::Custom(rects) => {
                Hitbox::new().with_rects(rects.clone())
            }
        };
        entity_builder = entity_builder
            .with(Collidable::new(CollisionTag::Enemy(enemy_type)))
            .with(Solid::new(SolidTag::Enemy(enemy_type)))
            .with(hitbox);
    }

    Ok(entity_builder.build())
}
