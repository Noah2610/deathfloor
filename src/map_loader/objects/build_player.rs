use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let player_settings = world.read_resource::<Settings>().player.clone();

    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;

    let entity = base_object_entity(world, object)?
        .with(Player::default())
        .with(Controllable::default())
        .with(Velocity::default())
        .with(Movable::default())
        .with(JumppadAffected::default())
        .with(HealthActionQueue::default())
        .with(AnimationEditor::default())
        .with(SoundPlayer::<SoundType>::default())
        .with(CanInteract::default())
        .with(Facing::default())
        .with(sprite_render)
        .build();

    if let Some(entity_config) = player_settings.entity_config {
        if let Err(e) = edit_entity_with_entity_config(
            world,
            entity,
            entity_config,
            object.variant(),
        ) {
            eprintln!("[WARNING]\n    Player entity config error:\n    {}", e);
        }
    }

    Ok(entity)
}
