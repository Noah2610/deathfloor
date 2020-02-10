use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let player_settings = world.read_resource::<SettingsRes>().0.player.clone();

    let size: Size = player_settings.size.into();
    let hitbox = Hitbox::new().with_rect(
        RectBuilder::from(&size)
            .top(size.h * 0.5 - 8.0)
            .build()
            .unwrap(),
    );
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;
    let movement_data = player_settings.movement;
    // TODO
    // let decr_velocity = DecreaseVelocity::from(movement_data.decr_velocity);
    // let gravity = Gravity::from(movement_data.gravity);
    // TODO
    let base_friction = BaseFriction::builder()
        .friction(&Axis::X, 5.0)
        .friction(&Axis::Y, 1.0)
        .build()
        .unwrap();
    dbg!(&base_friction);

    let entity = base_object_entity(world, object)?
        .with(Player::default())
        .with(Velocity::default())
        .with(size)
        .with(sprite_render)
        // .with(decr_velocity)
        // .with(gravity)
        .with(movement_data)
        .with(hitbox)
        .with(Collider::new(CollisionTag::Player))
        .with(Solid::new(SolidTag::Player))
        .with(base_friction)
        .build();

    Ok(entity)
}
