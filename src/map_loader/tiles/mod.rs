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

        // Config file settings
        if let Some(tile_settings) = tiles_settings.types.get(&tile.tile_type) {
            entity =
                edit_entity_with_tile_settings(entity, tile_settings, &size);
        }

        // Prop settings
        if let Ok(mut tile_settings) = TileSettings::try_from(tile.props()) {
            tile_settings.hitbox = tile.hitbox.map(HitboxConfig::from);
            entity =
                edit_entity_with_tile_settings(entity, &tile_settings, &size);
        }

        entity.build();
    }

    Ok(())
}
