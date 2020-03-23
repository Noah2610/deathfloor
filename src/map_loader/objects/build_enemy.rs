use super::helpers::prelude::*;

/// Builds the enemy entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let entity = base_object_entity(world, object)?
        .with(Loadable::default())
        .with(Hidden)
        .build();
    Ok(entity)
}
