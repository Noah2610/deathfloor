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
        ObjectType::Player => eprintln!(
            "[WARNING]
    Cannot build Player object here."
        ),

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
    for object in objects {
        if let ObjectType::Player = &object.object_type {
            let entity = build_player::build(world, &object)?;
            let _ = build_camera::build(world, level_data, Some(entity))?;
        } else {
            load_object(world, object)?;
        }
    }

    Ok(())
}
