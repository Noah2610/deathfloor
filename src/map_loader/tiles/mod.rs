mod helpers;

use helpers::prelude::*;
use std::convert::TryFrom;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: &TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    for tile in tiles {
        let tiles_settings =
            world.read_resource::<SettingsRes>().0.tiles.clone();
        let tile_type = TileType::try_from(tile.tile_type.as_str())?;

        let sprite_render = get_sprite_render(
            world,
            format!("spritesheets/tiles/{}", tile.ts),
            tile.id,
        )?;

        let mut entity =
            base_tile_entity(world, tile, tile_size)?.with(sprite_render);

        if let Some(tile_settings) = tiles_settings.types.get(&tile_type) {
            if tile.is_solid() {
                let body = tile_settings
                    .physics
                    .rigid_body()
                    .translation(tile.pos.into());
                let collider = tile_settings.physics.collider();
                entity = entity
                    .with_body::<f32, _>(body.build())
                    .with_collider::<f32>(&collider);
            }
        }

        entity.build();
    }

    Ok(())
}
