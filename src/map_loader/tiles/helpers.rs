pub(super) mod prelude {
    pub(in super::super) use super::base_tile_entity;
    // pub(in super::super) use super::edit_entity_with_tile_settings;
    pub use crate::map_loader::helpers::prelude::*;
}

use prelude::*;

/// Adds components depending on given `TileSettings` to `EntityBuilder`.
// pub(super) fn edit_entity_with_tile_settings<'a>(
//     mut entity: EntityBuilder<'a>,
//     tile_settings: &TileSettings,
//     size: &Size,
// ) -> EntityBuilder<'a> {
//     // HITBOX
//     if let Some(hitbox_type) = &tile_settings.hitbox {
//         let hitbox = match hitbox_type {
//             HitboxConfig::Size => Hitbox::new().with_rect(size.into()),
//             HitboxConfig::Custom(rects) => {
//                 Hitbox::new().with_rects(rects.clone())
//             }
//         };
//         entity = entity.with(hitbox);
//     }

//     entity
// }

pub(in super::super) static TILE_LOADABLE_PADDING: (Option<f32>, Option<f32>) =
    (Some(32.0), Some(32.0));

/// Adds base components to tile entity.
/// Components include:
///     - `Tile` // TODO
///     - `Transform`
///     - `Size`
///     - `ScaleOnce`
///     - `Transparent`
///     - `Loadable` (with padding)
///     - `Hidden`
///       Hidden, because the `EntityLoaderSystem` will add/remove
///       the Hidden component, depending on when it is loaded.
///     - `Lifecycle`
pub(super) fn base_tile_entity<'a>(
    world: &'a mut World,
    tile: &TileData,
    tile_size: SizeData,
) -> amethyst::Result<EntityBuilder<'a>> {
    let size: Size = tile_size.into();
    let loadable = Loadable::default().with_padding(TILE_LOADABLE_PADDING);

    Ok(base_entity(world, tile)?
        // .with(Tile::default()) // TODO
        .with(size)
        .with(loadable)
        .with(Hidden))
}
