pub(super) mod prelude {
    pub(in super::super) use super::base_tile_entity;
    pub use crate::map_loader::helpers::prelude::*;
}

use prelude::*;

/// Adds base components to tile entity.
/// Components include:
///     - `Tile` // TODO
///     - `Transform`
///     - `Size`
///     - `ScaleOnce`
///     - `Transparent`
///     - `Loadable`
///     - `Hidden`
///       Hidden, because the `EntityLoaderSystem` will add/remove
///       the Hidden component, depending on when it is loaded.
pub(super) fn base_tile_entity<'a>(
    world: &'a mut World,
    tile: &TileData,
    tile_size: SizeData,
) -> amethyst::Result<EntityBuilder<'a>> {
    const DEFAULT_Z: f32 = 0.0;

    let mut transform: Transform = tile.pos.into();
    transform.set_translation_z(tile.z_or(DEFAULT_Z));

    let size: Size = tile_size.into();

    let entity = world
        .create_entity()
        // .with(Tile::default()) // TODO
        .with(transform)
        .with(size)
        .with(ScaleOnce::default())
        .with(Transparent)
        .with(Loadable)
        .with(Hidden);

    Ok(entity)
}
