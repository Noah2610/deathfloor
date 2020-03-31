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

    let size = enemy_settings
        .components
        .as_ref()
        .and_then(|comps| comps.size.clone())
        .unwrap_or(object.size.into());

    // CREATE ENTITY_BUILDER

    let mut entity_builder = base_object_entity(world, object)?
        .with(Enemy::new(enemy_type))
        .with(
            Loadable::default(), /*.with_padding((
                                     // TILE_LOADABLE_PADDING.0.map(|x| -x),
                                     // TILE_LOADABLE_PADDING.1.map(|y| -y),
                                     // Some(-size.w),
                                     // Some(-size.h),
                                 ))*/
        )
        .with(Hidden)
        .with(sprite_render)
        .with(Velocity::default())
        .with(Movable::default());

    // COLLISION / SOLID TAGS

    if let Some(collision_tag) = enemy_settings.collision_tag {
        entity_builder = entity_builder
            .with(Collider::new(CollisionTag::from(collision_tag.clone())))
            .with(Collidable::new(CollisionTag::from(collision_tag)));
    }
    if let Some(solid_tag) = enemy_settings.solid_tag {
        entity_builder =
            entity_builder.with(Solid::new(CollisionTag::from(solid_tag)));
    }

    // COMPONENTS

    if let Some(components) = enemy_settings.components {
        entity_builder = entity_builder.with(size.clone());
        if let Some(gravity) = components.gravity {
            entity_builder = entity_builder.with(gravity);
        }
        if let Some(max_movement_velocity) = components.max_movement_velocity {
            entity_builder = entity_builder.with(max_movement_velocity);
        }
        if let Some(base_friction) = components.base_friction {
            entity_builder = entity_builder.with(base_friction);
        }
        if let Some(animations) = components.animations {
            entity_builder = entity_builder.with(animations);
        }
        if let Some(hitbox_config) = components.hitbox {
            let hitbox = match hitbox_config {
                HitboxConfig::Size => {
                    Hitbox::new().with_rect(Rect::from(&size))
                }
                HitboxConfig::Custom(rects) => {
                    Hitbox::new().with_rects(rects.clone())
                }
            };

            entity_builder =
                entity_builder.with(JumppadAffected::default()).with(hitbox);
        }
        if let Some(walker) = components.walker {
            entity_builder = entity_builder.with(walker);
        }
        if let Some(jumppad) = components.jumppad {
            entity_builder = entity_builder.with(jumppad);
        }
    }

    // EVENTS

    if let Some(events_register) = enemy_settings.events {
        entity_builder = entity_builder
            .with(events_register)
            .with(ActionTypeTrigger::default());
    }

    Ok(entity_builder.build())
}
