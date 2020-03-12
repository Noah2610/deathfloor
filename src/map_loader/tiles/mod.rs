pub mod tile_type;

mod helpers;

use helpers::prelude::*;

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

        if let Some(tile_settings) = tiles_settings.types.get(&tile.tile_type) {
            // HITBOX
            if let Some(hitbox_type) = &tile_settings.hitbox {
                let hitbox = match hitbox_type {
                    HitboxConfig::Size => {
                        Hitbox::new().with_rect((&size).into())
                    }
                    HitboxConfig::Custom(rects) => {
                        Hitbox::new().with_rects(rects.clone())
                    }
                };
                entity = entity.with(hitbox);
            }

            // SOLID
            if tile_settings.is_solid {
                entity = entity
                    .with(Collidable::new(CollisionTag::Tile))
                    .with(Solid::new(SolidTag::Tile));
            }

            // JUMPPAD
            if let Some(jumppad) = tile_settings.jumppad.as_ref().cloned() {
                entity = entity
                    .with(Collidable::new(CollisionTag::Jumppad))
                    .with(jumppad);
            }
        }

        // SOLID and HITBOX
        if tile
            .props()
            .get("is_solid")
            .and_then(|p| p.as_bool())
            .unwrap_or(false)
        {
            if let Some(hitbox) = tile.hitbox {
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
