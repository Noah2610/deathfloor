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

    let body = physics_data
        .rigid_body()
        .translation(pos)
        .user_data(crate::components::player::PlayerData {
            acceleration: player_settings.acceleration,
        })
        .build();
    let collider = physics_data.collider();

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
