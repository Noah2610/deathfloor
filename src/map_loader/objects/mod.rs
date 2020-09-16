pub mod object_type;

mod build_camera;
mod build_custom;
mod build_enemy;
mod build_player;
mod build_player_bullet;
mod helpers;

use super::map_data::prelude::*;
use amethyst::ecs::World;
use deathframe::amethyst;
use object_type::ObjectType;

pub fn load_object(
    world: &mut World,
    object: ObjectData,
) -> amethyst::Result<()> {
    match &object.object_type {
        ObjectType::Player => panic!("Cannot build Player object here."),
        ObjectType::CameraPath => panic!("Cannot build CameraPath here."),

        ObjectType::PlayerBullet => {
            let _ = build_player_bullet::build(world, &object);
        }

        ObjectType::Enemy(enemy_type) => {
            let _ = build_enemy::build(world, &object, enemy_type.clone())?;
        }

        ObjectType::Custom(custom_type) => {
            let _ = build_custom::build(world, &object, custom_type.clone())?;
        }

        ObjectType::None => eprintln!(
            "[WARNING]
    Object without a 'type' does nothing!
    Skipping loading of type-less object..."
        ),
    }

    Ok(())
}

pub(super) fn load_objects(
    world: &mut World,
    objects: ObjectsData,
    level_data: &LevelData,
) -> amethyst::Result<()> {
    let mut camera = None;
    let mut camera_path = None;

    for object in objects {
        if let ObjectType::Player = &object.object_type {
            let entity = build_player::build(world, &object)?;
            camera =
                Some(build_camera::build(world, level_data, Some(entity))?);
        } else if let ObjectType::CameraPath = &object.object_type {
            if let Some(polygon) = object.polygon {
                camera_path = Some(polygon);
            } else {
                eprintln!(
                    "[WARNING]\n    Object CameraPath needs to have a polygon \
                     to define the camera's path."
                );
            }
        } else {
            load_object(world, object)?;
        }
    }

    match (camera, camera_path) {
        (Some(camera), Some(camera_path)) => {
            add_camera_path_to_camera(world, camera, camera_path)?;
        }
        (Some(_), None) | (None, None) => (),
        (None, Some(_)) => {
            panic!(
                "Can't create camera path without Camera entity. Camera \
                 entity wasn't built."
            );
        }
    }

    Ok(())
}

fn add_camera_path_to_camera(
    world: &mut World,
    camera: deathframe::amethyst::ecs::Entity,
    camera_path: ObjectPolygonData,
) -> amethyst::Result<()> {
    use crate::components::prelude::LockedToPath;
    use deathframe::amethyst::ecs::WorldExt;

    world
        .write_storage::<LockedToPath>()
        .insert(camera, LockedToPath::from(camera_path))?;
    Ok(())
}
