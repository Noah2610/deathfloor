pub mod tile_type;

pub(super) mod helpers;

use helpers::prelude::*;
use std::convert::TryFrom;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    let tiles_settings = &world.read_resource::<Settings>().tiles.clone();

    for tile in tiles {
        let sprite_render = get_sprite_render(
            world,
            format!("spritesheets/tiles/{}", tile.ts),
            tile.id,
        )?;

        let entity_builder =
            base_tile_entity(world, &tile, tile_size)?.with(sprite_render);

        let mut tile_settings = TileSettings::default();

        // Config file settings
        if let Some(config_tile_settings) =
            tiles_settings.types.get(&tile.tile_type).cloned()
        {
            tile_settings.merge(config_tile_settings);
        }

        // Prop settings
        if let Ok(prop_tile_settings) = TileSettings::try_from(tile.props()) {
            tile_settings.merge(prop_tile_settings);
        }

        let entity = entity_builder.build();

        if let Some(entity_config) = tile_settings.entity {
            let type_variant = tile
                .props
                .get("variant")
                .and_then(|val| val.as_str())
                .map(ToString::to_string);

            edit_entity_with_entity_config(
                world,
                entity,
                entity_config,
                type_variant,
            )?;
        }
    }

    Ok(())
}
