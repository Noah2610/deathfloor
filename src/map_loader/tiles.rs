use super::map_data::prelude::*;
use crate::components::prelude::*;
use crate::helpers::resource;
use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::amethyst;
use deathframe::handles::SpriteSheetHandles;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: &TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    const DEFAULT_Z: f32 = 0.0;

    let size: Size = tile_size.into();

    for tile in tiles {
        let mut transform: Transform = tile.pos.into();
        transform.set_translation_z(tile.z_or(DEFAULT_Z));

        let sprite_render_opt = {
            let spritesheet_path =
                resource(format!("spritesheets/tiles/{}", tile.ts));
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

        // TODO
        // if tile.is_solid() {
        //     entity = entity.with(Solid::new(SolidTag::Tile));
        // }

        if let Some(sprite_render) = sprite_render_opt {
            entity = entity.with(sprite_render);
        }

        entity.build();
    }

    Ok(())
}
