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
    let base_friction = BaseFriction::from(physics_data.base_friction);
    let gravity = Gravity::from(physics_data.gravity);
    let max_movement_velocity = {
        let mut builder = MaxMovementVelocity::builder();
        for axis in Axis::iter() {
            let max_opt = physics_data.max_velocity.by_axis(&axis);
            builder = builder.with_opt(&axis, max_opt);
        }
        builder.build().unwrap()
    };

    let animations_container = player_settings.animations;

    let shooter = Shooter::from(player_settings.shooter);

    let mut entity_builder = base_object_entity(world, object)?
        .with(Player::default())
        .with(Velocity::default())
        .with(Movable::default())
        .with(player_settings.jumper)
        .with(max_movement_velocity)
        .with(sprite_render)
        .with(physics_data)
        .with(base_friction)
        .with(animations_container)
        .with(shooter)
        .with(JumppadAffected::default());

    if let Some(wall_jumper) = player_settings.wall_jumper {
        entity_builder = entity_builder.with(wall_jumper);
    }

    if let Some(wall_slider) = player_settings.wall_slider {
        entity_builder = entity_builder.with(wall_slider);
    }

    if let Some(hitbox_config) = &player_settings.hitbox {
        let hitbox = match hitbox_config {
            HitboxConfig::Size => Hitbox::new().with_rect(Rect::from(&size)),
            HitboxConfig::Custom(rects) => {
                Hitbox::new().with_rects(rects.clone())
            }
        };

        let collision_tag = CollisionTag::builder()
            .label(CollisionLabel::Player)
            .collides_with({
                use CollisionLabel::*;
                vec![Tile, Jumppad, Enemy, Bullet]
            }) // TODO: extract into player settings RON
            .build()
            .unwrap();
        let solid_tag = SolidTag::builder()
            .label(CollisionLabel::Player)
            .collides_with({
                use CollisionLabel::*;
                vec![Tile, Enemy]
            }) // TODO: extract into player settings RON
            .build()
            .unwrap();

        entity_builder = entity_builder
            .with(Collider::new(collision_tag.clone()))
            .with(Collidable::new(collision_tag))
            .with(Solid::new(solid_tag))
            .with(hitbox);
    }

    let entity = entity_builder.with(size).with(gravity).build();

    Ok(entity)
}
