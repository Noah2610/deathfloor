use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let player_settings = world.read_resource::<SettingsRes>().0.player.clone();

    let size: Size = player_settings.size.into();
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;
    let physics_data = player_settings.physics;

    // TODO
    // let decr_velocity = DecreaseVelocity::from(physics_data.decr_velocity);
    // let gravity = Gravity::from(physics_data.gravity);

    const DEFAULT_Z: f32 = 1.0;
    let mut transform: Transform = object.pos.into();
    transform.set_translation_z(object.z_or(DEFAULT_Z));

    let pos = {
        let translation = transform.translation();
        Vector::new(translation.x, translation.y)
    };
    // let body = RigidBodyDesc::<f32>::new()
    //     .translation(pos)
    //     .gravity_enabled(true)
    //     .mass(physics_data.mass)
    //     .status(BodyStatus::Dynamic)
    //     .linear_damping(physics_data.damping) // NOTE: This is DecreaseVelocity
    //     .build();
    // let shape =
    //     ShapeHandle::new(Cuboid::new(Vector::new(size.w * 0.5, size.h * 0.5)));
    // let collider = ColliderDesc::new(shape);

    let body = physics_data.rigid_body().translation(pos).build();
    let collider = physics_data.collider();

    let entity = base_object_entity(world, object)?
        .with_body::<f32, _>(body)
        .with_collider::<f32>(&collider)
        .with(transform)
        .with(Player::default())
        // .with(Velocity::default())
        .with(size)
        .with(sprite_render)
        // .with(decr_velocity)
        // .with(gravity)
        // .with(physics_data)
        // .with(Solid::new(SolidTag::Player))
        .build();

    Ok(entity)
}
