mod build_camera;
mod build_player;
mod helpers;
mod object_type;

use super::map_data::prelude::*;
use amethyst::ecs::World;
use deathframe::amethyst;
use object_type::ObjectType;
use std::convert::TryFrom;

pub(super) fn load_objects(
    world: &mut World,
    objects: ObjectsData,
    level_data: &LevelData,
) -> amethyst::Result<()> {
    for object in objects {
        let object_type = ObjectType::try_from(object.object_type.as_str())?;

        match object_type {
            ObjectType::Player => {
                let entity = build_player::build(world, &object)?;
                build_camera::build(world, level_data, Some(entity))?;
            }
        }
    }

    Ok(())
}
