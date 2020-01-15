mod map_data;

use amethyst::ecs::{World, WorldExt};
use deathframe::amethyst;
use map_data::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn load_map_data<T>(
    map_filepath: T,
    world: &mut World,
) -> amethyst::Result<Map>
where
    T: Into<PathBuf>,
{
    let map_filepath = map_filepath.into();
    let map_file = File::open(map_filepath)?;
    let map = serde_json::from_reader::<_, Map>(map_file)?;
    dbg!(&map);

    unimplemented!()
}
