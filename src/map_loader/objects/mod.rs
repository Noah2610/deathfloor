mod build_player;
mod helpers;
mod object_type;

use super::map_data::prelude::*;
use crate::components::prelude::*;
use crate::helpers::resource;
use amethyst::ecs::{EntityBuilder, World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::amethyst;
use deathframe::handles::SpriteSheetHandles;
use object_type::ObjectType;
use std::convert::TryFrom;

pub(super) fn load_objects(
    world: &mut World,
    objects: &ObjectsData,
) -> amethyst::Result<()> {
    const DEFAULT_Z: f32 = 0.1;

    for object in objects {
        let mut transform: Transform = object.pos.into();
        transform.set_translation_z(object.z_or(DEFAULT_Z));

        let size: Size = object.size.into();

        let object_type = ObjectType::try_from(object.object_type.as_str())?;

        match object_type {
            ObjectType::Player => {
                let entity = build_player::build(world, object)?;
                create_camera(world, Some(entity));
            }
        }
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
