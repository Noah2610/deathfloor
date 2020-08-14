pub(super) mod prelude {
    pub(in super::super) use super::base_object_entity;
    pub use crate::map_loader::helpers::prelude::*;
}

use prelude::*;

/// Adds base components to object entity.
/// Components include:
///     - `Transform`
///     - `Size`
///     - `ScaleOnce`
///     - `Transparent`
///     - `Lifecycle`
pub(super) fn base_object_entity<'a>(
    world: &'a mut World,
    object: &ObjectData,
) -> amethyst::Result<EntityBuilder<'a>> {
    let size: Size = object.size.into();

    Ok(base_entity(world, object)?
        .with(size)
        .with(Lifecycle::default()))
}
