use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let player_settings = world.read_resource::<Settings>().player.clone();

    let size: Size = player_settings.size.into();
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;
    let physics_data = player_settings.physics;
    let max_movement_velocity = {
        let mut builder = MaxMovementVelocity::builder();
        for axis in Axis::iter() {
            let max_opt = physics_data.max_velocity.by_axis(&axis);
            builder = builder.with_opt(&axis, max_opt);
        }
        builder.build().unwrap()
    };

    let hitbox = match player_settings.hitbox {
        HitboxConfig::Size => Hitbox::new().with_rect(Rect::from(&size)),
        HitboxConfig::Custom(rects) => Hitbox::new().with_rects(rects),
    };

    let collision_tag = CollisionTag::from(player_settings.collision_tag);
    let solid_tag = SolidTag::from(player_settings.solid_tag);

    let mut entity_builder = base_object_entity(world, object)?
        .with(Player::default())
        .with(Velocity::default())
        .with(Movable::default())
        .with(JumppadAffected::default())
        .with(Collider::new(collision_tag.clone()))
        .with(Collidable::new(collision_tag))
        .with(Solid::new(solid_tag))
        .with(Shooter::from(player_settings.shooter))
        .with(Gravity::from(physics_data.gravity))
        .with(BaseFriction::from(physics_data.base_friction))
        .with(player_settings.jumper)
        .with(player_settings.animations)
        .with(player_settings.health)
        .with(player_settings.health_display)
        .with(player_settings.takes_damage)
        .with(hitbox)
        .with(size)
        .with(max_movement_velocity)
        .with(sprite_render)
        .with(physics_data);

    if let Some(wall_jumper) = player_settings.wall_jumper {
        entity_builder = entity_builder.with(wall_jumper);
    }

    if let Some(wall_slider) = player_settings.wall_slider {
        entity_builder = entity_builder.with(wall_slider);
    }

    Ok(entity_builder.build())
}
