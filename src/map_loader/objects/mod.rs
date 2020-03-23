pub mod object_type;

mod build_camera;
mod build_enemy;
mod build_player;
mod helpers;

use super::map_data::prelude::*;
use amethyst::ecs::World;
use deathframe::amethyst;
use object_type::ObjectType;

pub(super) fn load_objects(
    world: &mut World,
    objects: ObjectsData,
    level_data: &LevelData,
) -> amethyst::Result<()> {
    for object in objects {
        match object.object_type {
            ObjectType::Player => {
                let entity = build_player::build(world, &object)?;
                let _ = build_camera::build(world, level_data, Some(entity))?;
            }

            ObjectType::Enemy(enemy_type) => {
                let _ = build_enemy::build(world, &object, enemy_type)?;
            }

            ObjectType::None => eprintln!(
                "WARNING:
    Object without a 'type' does nothing!
    Skipping loading of type-less object..."
            ),
        }
    }

    Ok(())
}
