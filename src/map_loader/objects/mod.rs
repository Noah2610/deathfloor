use super::map_data::prelude::*;
use crate::components::prelude::*;
use crate::helpers::resource;
use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::amethyst;
use deathframe::handles::SpriteSheetHandles;

pub(super) fn load_objects(
    world: &mut World,
    objects: &ObjectsData,
) -> amethyst::Result<()> {
    const DEFAULT_Z: f32 = 0.1;

    for object in objects {
        let mut transform: Transform = object.pos.into();
        transform.set_translation_z(object.z_or(DEFAULT_Z));

        let size: Size = object.size.into();

        let sprite_render_opt = {
            let (handle, number) = {
                let (path, number) = match object.object_type.as_str() {
                    "Player" => ("spritesheets/player.png", 1),
                    t => {
                        return Err(amethyst::Error::from_string(format!(
                            "Invalid object type: {}",
                            t
                        )))
                    }
                };
                let handle = world
                    .write_resource::<SpriteSheetHandles>()
                    .get_or_load(resource(path), world);
                (handle, number)
            };
            Some(SpriteRender {
                sprite_sheet:  handle.clone(),
                sprite_number: number,
            })
        };

        let mut entity = world
            .create_entity()
            .with(transform)
            .with(size.clone())
            .with(ScaleOnce::default())
            .with(Transparent);

        if object.is_solid() {
            entity = entity.with(Solid::new(SolidTag::Tile));
        }

        if let Some(sprite_render) = sprite_render_opt {
            entity = entity.with(sprite_render);
        }

        let entity = entity.build();
        create_camera(world, Some(entity));
    }

    Ok(())
}

// TODO
use amethyst::ecs::Entity;
use amethyst::renderer::camera::Camera;

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

    if let Some(player_entity) = player_entity_opt {
        camera = camera.with(Follow::new(player_entity));
    }

    camera.build();
}
