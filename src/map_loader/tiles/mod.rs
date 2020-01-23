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
            if tile_settings.solid {
                let collider = tile_settings.physics.collider();
                // .translation(tile.pos.into());
                entity = entity
                    .with(Solid::default())
                    // .with_body::<f32, _>(body)
                    .with_collider::<f32>(&collider);

                let body = tile_settings.physics.body();

                match body.downcast::<Ground>() {
                    Ok(ground_body) => {
                        entity = entity.with_body::<f32, _>(*ground_body);
                    }
                    Err(body) => {
                        if let Ok(rigid_body) = body.downcast::<RigidBody>() {
                            let mut rigid = *rigid_body;
                            rigid.set_position(Isometry2::new(
                                tile.pos.into(),
                                0.0,
                            ));
                            entity = entity.with_body::<f32, _>(rigid);
                        } else {
                            return Err(amethyst::Error::from_string(
                                "Solid tile's body must be Ground or RigidBody",
                            ));
                        }
                    }
                }
                // body.set_position();
                // body.translation(tile.pos.into());
            }
        }

        entity.build();
    }
    Ok(())
}
