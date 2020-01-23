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

    const DEFAULT_Z: f32 = 1.0;
    let mut transform: Transform = object.pos.into();
    transform.set_translation_z(object.z_or(DEFAULT_Z));

    let pos = {
        let translation = transform.translation();
        Vector::new(translation.x, translation.y)
    };

    let mut body =
        *physics_data.body().downcast::<RigidBody>().map_err(|_| {
            amethyst::Error::from_string("Player should have a RigidBody body")
        })?;

    body.set_position(Isometry2::new(pos, 0.0));

    body.set_user_data(Some(Box::new(crate::components::player::PlayerData {
        acceleration:  player_settings.acceleration,
        jump_strength: player_settings.jump_strength,
    })));

    // .translation(pos)
    // .user_data()
    // .build();

    let collider = physics_data.collider();
    // let collider = physics_data.collider().translation(pos);

    let grounded =
        Grounded::default().with_padding(player_settings.grounded_padding);

    let entity = base_object_entity(world, object)?
        .with_body::<f32, _>(body)
        .with_collider::<f32>(&collider)
        .with(transform)
        .with(Player::default())
        .with(size)
        .with(sprite_render)
        .with(grounded)
        .build();

    Ok(entity)
}
