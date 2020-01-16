use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;

    let entity = base_object_entity(world, object)?
        .with(sprite_render)
        .with(Solid::new(SolidTag::Player))
        .build();

    Ok(entity)
}

// TODO
// fn create_player(world: &mut World) -> Entity {
//     const PLAYER_Z: f32 = 1.0;
//     const PLAYER_SIZE: (f32, f32) = (32.0, 64.0);

//     let player_settings = world.read_resource::<SettingsRes>().0.player.clone();

//     let mut transform = Transform::default();
//     transform.set_translation_xyz(0.0, 0.0, PLAYER_Z);

//     let size = Size::from(PLAYER_SIZE);

//     let sprite_render = {
//         let mut spritesheet_handles =
//             world.write_resource::<SpriteSheetHandles>();
//         SpriteRender {
//             sprite_number: 1,
//             sprite_sheet:  spritesheet_handles
//                 .get_or_load(resource("spritesheets/player.png"), world),
//         }
//     };

//     let movement_data = player_settings.movement;

//     let decr_velocity = DecreaseVelocity::from(movement_data.decr_velocity);

//     // TODO GRAVITY
//     // let gravity = Gravity::from(movement_data.gravity);

//     world
//         .create_entity()
//         .with(Player::default())
//         .with(transform)
//         .with(Velocity::default())
//         .with(size)
//         .with(sprite_render)
//         .with(ScaleOnce::default())
//         .with(decr_velocity)
//         // .with(gravity) // TODO
//         .with(movement_data)
//         .build()
// }
