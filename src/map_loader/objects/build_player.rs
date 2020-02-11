use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let player_settings = world.read_resource::<SettingsRes>().0.player.clone();

    let size: Size = player_settings.size.into();
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;
    let movement_data = player_settings.movement;
    let base_friction = BaseFriction::from(movement_data.base_friction);
    // let decr_velocity = DecreaseVelocity::from(movement_data.decr_velocity);
    // let gravity = Gravity::from(movement_data.gravity);

    let mut entity_builder = base_object_entity(world, object)?
        .with(Player::default())
        .with(Velocity::default())
        .with(sprite_render)
        .with(movement_data)
        .with(base_friction);

    if let Some(hitbox_config) = &player_settings.hitbox {
        let hitbox = match hitbox_config {
            HitboxConfig::Size => Hitbox::new().with_rect(Rect::from(&size)),
            HitboxConfig::Custom(rects) => {
                Hitbox::new().with_rects(rects.clone())
            }
        };
        entity_builder = entity_builder
            .with(Collider::new(CollisionTag::Player))
            .with(Solid::new(SolidTag::Player))
            .with(hitbox);
    }

    let entity = entity_builder
        .with(size)
        // .with(decr_velocity)
        // .with(gravity)
        .build();

    Ok(entity)
}
