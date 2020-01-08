use super::state_prelude::*;
use crate::helpers::resource;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        let player_entity = create_player(data.world);
        create_camera(data.world, Some(player_entity));
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();
        Trans::None
    }
}

fn create_player(world: &mut World) -> Entity {
    const PLAYER_Z: f32 = 1.0;
    const PLAYER_SIZE: (f32, f32) = (32.0, 64.0);

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, PLAYER_Z);

    let size = Size::from(PLAYER_SIZE);

    let sprite_render = {
        use amethyst::renderer::SpriteRender;

        let mut spritesheet_handles =
            world.write_resource::<SpriteSheetHandles>();
        SpriteRender {
            sprite_number: 1,
            sprite_sheet:  spritesheet_handles
                .get_or_load(resource("spritesheets/player.png"), world),
        }
    };

    world
        .create_entity()
        .with(Player::default())
        .with(transform)
        .with(Velocity::default())
        .with(size)
        .with(sprite_render)
        .with(ScaleOnce::default())
        .build()
}

fn create_camera(world: &mut World, player_entity_opt: Option<Entity>) {
    use amethyst::utils::ortho_camera::{
        CameraNormalizeMode,
        CameraOrtho,
        CameraOrthoWorldCoordinates,
    };

    const CAMERA_Z: f32 = 10.0;
    const CAMERA_SIZE: (f32, f32) = (400.0, 400.0);

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, CAMERA_Z);

    let size = Size::from(CAMERA_SIZE);

    let camera = Camera::standard_2d(size.w, size.h);
    let mut camera_ortho =
        CameraOrtho::normalized(CameraNormalizeMode::Contain);
    camera_ortho.world_coordinates = {
        let half_size = (size.w * 0.5, size.h * 0.5);
        CameraOrthoWorldCoordinates {
            top:    half_size.1,
            bottom: -half_size.1,
            left:   -half_size.0,
            right:  half_size.0,
        }
    };

    let mut camera = world
        .create_entity()
        .with(transform)
        .with(size)
        .with(camera)
        .with(camera_ortho);

    // TODO
    // if let Some(player_entity) = player_entity_opt {
    //     camera = camera.with(Follow::new(player_entity));
    // }

    camera.build();
}
