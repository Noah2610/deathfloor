mod map_data;

use crate::components::prelude::*;
use crate::helpers::resource;
use amethyst::ecs::{EntityBuilder, World, WorldExt};
use amethyst::prelude::Builder;
use amethyst::renderer::SpriteRender;
use deathframe::amethyst;
use deathframe::handles::SpriteSheetHandles;
use map_data::prelude::*;
use std::fs::File;
use std::path::PathBuf;

pub fn load_map<T>(world: &mut World, map_filepath: T) -> amethyst::Result<()>
where
    T: Into<PathBuf>,
{
    let map = get_map_data(map_filepath)?;

    load_tiles(world, &map.tiles, map.level.tile_size)?;

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

fn load_tiles(
    world: &mut World,
    tiles: &TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    const DEFAULT_Z: f32 = 0.0;

    let size = Size::new(tile_size.w, tile_size.h);

    for tile in tiles {
        let mut transform = Transform::default();
        let z = tile
            .props
            .get("z")
            .and_then(|z_value| z_value.as_f64().map(|f| f as f32))
            .unwrap_or(DEFAULT_Z);
        transform.set_translation_xyz(tile.pos.x, tile.pos.y, z);

        let spritesheet_path =
            resource(format!("spritesheets/tiles/{}", tile.ts));

        let sprite_render_opt = {
            let spritesheet_handle = world
                .write_resource::<SpriteSheetHandles>()
                .get_or_load(&spritesheet_path, world);
            Some(SpriteRender {
                sprite_sheet:  spritesheet_handle.clone(),
                sprite_number: tile.id,
            })
        };

        let mut entity = world
            .create_entity()
            .with(transform)
            .with(size.clone())
            .with(ScaleOnce::default())
            .with(Transparent);

        if tile.is_solid() {
            entity = entity.with(Solid::new(SolidTag::Tile));
        }

        if let Some(sprite_render) = sprite_render_opt {
            entity = entity.with(sprite_render);
        }

        entity.build();
    }

    Ok(())
}

// TODO: Don't know about this function...
//       Seems really cool, but takes away a lot of flexibility
//       (see `Solid` component usage below).
// fn manipulate_propful_entity<'a, P>(
//     mut entity: EntityBuilder<'a>,
//     propful: &P,
// ) -> EntityBuilder<'a>
// where
//     P: Propful,
// {
//     if propful.is_solid() {
//         entity = entity.with(Solid::<SolidTag>::default());
//     }

//     entity
// }
