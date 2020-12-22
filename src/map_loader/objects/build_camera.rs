use super::helpers::prelude::*;
use amethyst::ecs::{Entities, Join, ReadStorage};
use amethyst::renderer::Camera;
use amethyst::utils::ortho_camera::{CameraNormalizeMode, CameraOrtho};

pub(super) fn build(
    world: &mut World,
    level_data: &LevelData,
    player_entity_opt: Option<Entity>,
) -> amethyst::Result<Entity> {
    let camera_settings = world.read_resource::<Settings>().camera.clone();

    let pos = {
        let default_camera_pos = (0.0, 0.0, camera_settings.z);
        if let Some(player_entity) = player_entity_opt {
            let player_entity_id = player_entity.id();
            let player_pos_opt = world.exec(
                |(entities, players, transforms): (
                    Entities,
                    ReadStorage<Player>,
                    ReadStorage<Transform>,
                )| {
                    (&entities, &players, &transforms).join().find_map(
                        |(entity, _, transform)| {
                            if entity.id() == player_entity_id {
                                let pos = transform.translation();
                                Some((pos.x, pos.y))
                            } else {
                                None
                            }
                        },
                    )
                },
            );
            if let Some(player_pos) = player_pos_opt {
                (player_pos.0, player_pos.1, camera_settings.z)
            } else {
                default_camera_pos
            }
        } else {
            default_camera_pos
        }
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(pos.0, pos.1, pos.2);

    let size: Size = camera_settings.size.into();

    let camera = Camera::standard_2d(size.w, size.h);
    let camera_ortho = CameraOrtho::new(
        CameraNormalizeMode::Contain,
        camera_settings.world_coordinates,
    );
    let half_size = (size.w * 0.5, size.h * 0.5);
    let loader = Loader::new(half_size.0, half_size.1);
    let confined = {
        let confined_rect = Rect::builder()
            .top(level_data.size.h)
            .bottom(0.0)
            .left(0.0)
            .right(level_data.size.w)
            .build()
            .unwrap();
        Confined::from(confined_rect)
    };

    let mut entity = world
        .create_entity()
        .with(transform)
        .with(size)
        .with(camera)
        .with(camera_ortho)
        .with(loader)
        .with(confined);

    if let Some(player_entity) = player_entity_opt {
        entity = entity.with(Follow::new(player_entity));
    }

    Ok(entity.build())
}
