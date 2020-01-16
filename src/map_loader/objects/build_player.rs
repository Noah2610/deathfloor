use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;

    let entity = base_object_entity(world, object)?
        .with(sprite_render)
        .with(Solid::new(SolidTag::Player))
        .build();

    Ok(entity)
}
