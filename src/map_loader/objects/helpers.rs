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
pub(super) fn base_object_entity<'a>(
    world: &'a mut World,
    object: &ObjectData,
) -> amethyst::Result<EntityBuilder<'a>> {
    const DEFAULT_Z: f32 = 1.0;

    let mut transform: Transform = object.pos.into();
    transform.set_translation_z(object.z_or(DEFAULT_Z));

    let size: Size = object.size.into();

    let entity = world
        .create_entity()
        .with(transform)
        .with(size.clone())
        .with(ScaleOnce::default())
        .with(Transparent);

    Ok(entity)
}
