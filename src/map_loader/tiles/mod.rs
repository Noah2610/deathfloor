pub mod tile_type;

mod helpers;

use helpers::prelude::*;
use std::convert::TryFrom;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    let tiles_settings = world.read_resource::<SettingsRes>().0.tiles.clone();
    let size: Size = tile_size.into();

    for tile in tiles {
        let sprite_render = get_sprite_render(
            world,
            format!("spritesheets/tiles/{}", tile.ts),
            tile.id,
        )?;

        let mut entity =
            base_tile_entity(world, &tile, tile_size)?.with(sprite_render);

        let mut tile_settings = TileSettings::default();

        // Config file settings
        if let Some(config_tile_settings) =
            tiles_settings.types.get(&tile.tile_type)
        {
            tile_settings = config_tile_settings.clone().merge(tile_settings);
        }

        // Prop settings
        if let Ok(mut prop_tile_settings) = TileSettings::try_from(tile.props())
        {
            prop_tile_settings.hitbox = tile.hitbox.map(HitboxConfig::from);
            tile_settings = prop_tile_settings.merge(tile_settings);
        }

        entity = edit_entity_with_tile_settings(entity, &tile_settings, &size);
        entity.build();
    }

    Ok(())
}
