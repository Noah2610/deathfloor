use super::helpers::prelude::*;

/// Builds the player entity.
pub(super) fn build(
    world: &mut World,
    object: &ObjectData,
) -> amethyst::Result<Entity> {
    let player_settings = world.read_resource::<SettingsRes>().0.player.clone();

    let size: Size = player_settings.size.into();
    let sprite_render = get_sprite_render(world, "spritesheets/player.png", 1)?;
    let movement_data = player_settings.movement;
    let base_friction = BaseFriction::from(movement_data.base_friction);
    let gravity = Gravity::from(movement_data.gravity);
    let max_movement_velocity = {
        let mut builder = MaxMovementVelocity::builder();
        for axis in Axis::iter() {
            let max_opt = movement_data.max_velocity.by_axis(&axis);
            builder = builder.with_opt(&axis, max_opt);
        }
        builder.build().unwrap()
    };

    let idle_iter = {
        let iter = vec![(13_usize, 500_u64).into(), (14_usize, 500_u64).into()]
            .into_iter();
        let rev = iter.clone().rev();
        iter.chain(rev).cycle()
    };
    let walk_iter = {
        vec![
            (1_usize, 100_u64).into(),
            (2_usize, 100_u64).into(),
            (3_usize, 100_u64).into(),
            (4_usize, 100_u64).into(),
            (5_usize, 100_u64).into(),
            (6_usize, 100_u64).into(),
            (7_usize, 100_u64).into(),
            (8_usize, 100_u64).into(),
            (9_usize, 100_u64).into(),
            (10_usize, 100_u64).into(),
            (12_usize, 100_u64).into(),
        ]
        .into_iter()
        .cycle()
    };

    let animations_container = AnimationsContainer::builder()
        .with(AnimationKey::Idle, move || Box::new(idle_iter.clone()))
        .with(AnimationKey::Walk, move || Box::new(walk_iter.clone()))
        .current(AnimationKey::Idle)
        .unwrap()
        .build()
        .unwrap();

    let mut entity_builder = base_object_entity(world, object)?
        .with(Player::default())
        .with(Velocity::default())
        .with(Movable::default())
        .with(max_movement_velocity)
        .with(sprite_render)
        .with(movement_data)
        .with(base_friction)
        .with(animations_container);

    if let Some(hitbox_config) = &player_settings.hitbox {
        let hitbox = match hitbox_config {
            HitboxConfig::Size => Hitbox::new().with_rect(Rect::from(&size)),
            HitboxConfig::Custom(rects) => {
                Hitbox::new().with_rects(rects.clone())
            }
        };
        entity_builder = entity_builder
            .with(Collider::new(CollisionTag::Player))
            .with(Solid::new(SolidTag::Player))
            .with(hitbox);
    }

    let entity = entity_builder.with(size).with(gravity).build();

    Ok(entity)
}
