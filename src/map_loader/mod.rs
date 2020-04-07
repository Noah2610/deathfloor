pub mod map_data;
pub mod types {
    pub use super::objects::object_type::ObjectType;
    pub use super::tiles::tile_type::TileType;
}

mod helpers;
mod objects;
mod tiles;

use amethyst::ecs::World;
use deathframe::amethyst;
use map_data::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn load_map<T>(world: &mut World, map_filepath: T) -> amethyst::Result<()>
where
    T: Into<PathBuf>,
{
    let map = get_map_data(map_filepath)?;

    tiles::load_tiles(world, map.tiles, map.level.tile_size)?;
    objects::load_objects(world, map.objects, &map.level)?;

    Ok(())
}

fn get_map_data<T>(map_filepath: T) -> amethyst::Result<MapData>
where
    T: Into<PathBuf>,
{
    let map_filepath = map_filepath.into();
    let map_file = File::open(map_filepath)?;
    let map = serde_json::from_reader(map_file)?;
    Ok(map)
}
