use super::helpers::prelude::*;

/// Builds the player bullet entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let settings = world.read_resource::<Settings>().player_bullet.clone();

    let size = settings
        .entity_config
        .as_ref()
        .and_then(|entity_config| {
            entity_config
                .components
                .as_ref()
                .and_then(|comps| comps.size.clone())
        })
        .unwrap_or(object.size.into());

    let sprite_render = get_sprite_render(
        world,
        format!("spritesheets/{}", settings.spritesheet_filename),
        0,
    )?;

    let entity = base_object_entity(world, object)?
        .with(size)
        .with(sprite_render)
        .with(Bullet::default())
        .build();

    if let Some(entity_config) = settings.entity_config {
        edit_entity_with_entity_config(
            world,
            entity,
            entity_config,
            object.variant(),
        )?;

        if let Some(velocity) =
            world.write_storage::<Velocity>().get_mut(entity)
        {
            let dir_x =
                object.props.get("dir_x").and_then(|v| v.as_f64()).expect(
                    "Spawned PlayerBullet object should have `dir_x` float \
                     prop",
                ) as f32;
            velocity.x *= dir_x;
        }
    }

    Ok(entity)
}
