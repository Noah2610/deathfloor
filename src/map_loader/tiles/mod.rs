mod helpers;

use helpers::prelude::*;
use std::convert::TryFrom;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    let size: Size = tile_size.into();

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
            base_tile_entity(world, &tile, tile_size)?.with(sprite_render);

        if let Some(hitbox) = tile.hitbox {
            entity = entity
                .with(hitbox)
                .with(Collidable::new(CollisionTag::Tile))
                .with(Solid::new(SolidTag::Tile));
        } else if let Some(tile_settings) = tiles_settings.types.get(&tile_type)
        {
            if let Some(hitbox_type) = &tile_settings.hitbox {
                let hitbox = match hitbox_type {
                    HitboxConfig::Size => {
                        Hitbox::new().with_rect((&size).into())
                    }
                    HitboxConfig::Custom(rects) => {
                        Hitbox::new().with_rects(rects.clone())
                    }
                };
                entity = entity
                    .with(hitbox)
                    .with(Collidable::new(CollisionTag::Tile))
                    .with(Solid::new(SolidTag::Tile));
            }
        }

        entity.build();
    }

    Ok(())
}
