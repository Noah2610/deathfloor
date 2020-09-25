use super::system_prelude::*;

pub fn insert_components(
    entity: Entity,
    components: EntityComponentsData,
    mut storages: &mut EntityComponentsStorages,
) -> amethyst::Result<()> {
    let EntityComponentsData {
        size,
        velocity,
        gravity,
        max_movement_velocity,
        movement_acceleration,
        base_friction,
        animation,
        animations,
        hitbox,
        walker,
        jumppad,
        scale_once,
        health,
        health_display,
        deals_damage,
        takes_damage,
        bullet,
        ledge_detector_data,
        death_on_contact,
        death_after_delay,
        interactable,
        facing,
        jumper,
        wall_jumper,
        wall_slider,
        shooter,
        kill_velocity_min,
    } = components;
    let &mut EntityComponentsStorages {
        entities,
        transform: transform_store,
        size: size_store,
        velocity: velocity_store,
        gravity: gravity_store,
        max_movement_velocity: max_movement_velocity_store,
        movement_acceleration: movement_acceleration_store,
        base_friction: base_friction_store,
        animation: animation_store,
        animations: animations_store,
        animation_editor: animation_editor_store,
        hitbox: hitbox_store,
        collider: collider_store,
        collidable: collidable_store,
        solid: solid_store,
        walker: walker_store,
        jumppad: jumppad_store,
        jumppad_affected: jumppad_affected_store,
        scale_once: scale_once_store,
        health: health_store,
        health_action_queue: health_action_queue_store,
        health_display: health_display_store,
        deals_damage: deals_damage_store,
        takes_damage: takes_damage_store,
        bullet: bullet_store,
        ledge_detector: ledge_detector_store,
        ledge_detector_corner_detector: ledge_detector_corner_detector_store,
        follow: follow_store,
        death_bound: death_bound_store,
        death_on_contact: death_on_contact_store,
        death_after_delay: death_after_delay_store,
        interactable: interactable_store,
        facing: facing_store,
        jumper: jumper_store,
        wall_jumper: wall_jumper_store,
        wall_slider: wall_slider_store,
        shooter: shooter_store,
        kill_velocity_min: kill_velocity_min_store,
    } = &mut storages;

    let size_opt = size.or_else(|| size_store.get(entity).cloned());

    if let Some(velocity) = velocity {
        velocity_store.insert(entity, velocity)?;
    }
    if let Some(gravity) = gravity {
        gravity_store.insert(entity, gravity)?;
    }
    if let Some(max_movement_velocity) = max_movement_velocity {
        max_movement_velocity_store.insert(entity, max_movement_velocity)?;
    }
    if let Some(movement_acceleration) = movement_acceleration {
        movement_acceleration_store.insert(entity, movement_acceleration)?;
    }
    if let Some(base_friction) = base_friction {
        base_friction_store.insert(entity, base_friction)?;
    }
    if let Some(mut animation) = animation {
        animation.play_cycle();
        animation_store.insert(entity, animation)?;
    }
    if let Some(animations) = animations {
        animations_store.insert(entity, animations)?;
        animation_editor_store.insert(entity, AnimationEditor::default())?;
    }
    if let Some(hitbox_config) = hitbox {
        let hitbox = match hitbox_config {
            HitboxConfig::Size => {
                if let Some(size) = size_opt.as_ref() {
                    Hitbox::new().with_rect(Rect::from(size))
                } else {
                    panic!(
                        "Cannot create `Hitbox` with `HitboxConfig::Size` \
                         because entity has no size!"
                    )
                }
            }
            HitboxConfig::Custom(rects) => {
                Hitbox::new().with_rects(rects.clone())
            }
        };

        hitbox_store.insert(entity, hitbox)?;
        jumppad_affected_store.insert(entity, JumppadAffected::default())?;
    }
    if let Some(walker) = walker {
        walker_store.insert(entity, walker)?;
    }
    if let Some(jumppad) = jumppad {
        jumppad_store.insert(entity, jumppad)?;
    }
    if let Some(scale_once) = scale_once {
        scale_once_store.insert(entity, scale_once)?;
    }
    if let Some(health) = health {
        health_store.insert(entity, health)?;
        health_action_queue_store
            .insert(entity, HealthActionQueue::default())?;
    }
    if let Some(health_display) = health_display {
        health_display_store.insert(entity, health_display)?;
    }
    if let Some(deals_damage) = deals_damage {
        deals_damage_store.insert(entity, deals_damage)?;
    }
    if let Some(takes_damage) = takes_damage {
        takes_damage_store.insert(entity, takes_damage)?;
    }
    if let Some(bullet) = bullet {
        bullet_store.insert(entity, bullet)?;
    }
    if let Some(ledge_detector_data) = ledge_detector_data {
        insert_ledge_detector(
            entity,
            ledge_detector_data,
            size_opt.as_ref().ok_or_else(|| {
                amethyst::Error::from_string(String::from(
                    "LedgeDetector entity needs to have a size",
                ))
            })?,
            (
                entities,
                ledge_detector_store,
                ledge_detector_corner_detector_store,
                transform_store,
                size_store,
                hitbox_store,
                collider_store,
                follow_store,
                death_bound_store,
            ),
        )?;
    }
    if let Some(death_on_contact) = death_on_contact {
        death_on_contact_store.insert(entity, death_on_contact)?;
    }
    if let Some(death_after_delay) = death_after_delay {
        death_after_delay_store.insert(entity, death_after_delay)?;
    }
    if let Some(interactable) = interactable {
        interactable_store.insert(entity, interactable)?;
    }
    if let Some(facing) = facing {
        facing_store.insert(entity, facing)?;
    }
    if let Some(jumper) = jumper {
        jumper_store.insert(entity, jumper)?;
    }
    if let Some(wall_jumper) = wall_jumper {
        wall_jumper_store.insert(entity, wall_jumper)?;
    }
    if let Some(wall_slider) = wall_slider {
        wall_slider_store.insert(entity, wall_slider)?;
    }
    if let Some(shooter) = shooter {
        shooter_store.insert(entity, shooter)?;
    }
    if let Some(kill_velocity_min) = kill_velocity_min {
        kill_velocity_min_store.insert(entity, kill_velocity_min)?;
    }
    if let Some(size) = size_opt {
        size_store.insert(entity, size)?;
    }

    Ok(())
}

fn insert_ledge_detector(
    entity: Entity,
    ledge_detector_data: LedgeDetectorData,
    owner_size: &Size,
    (
        entities,
        ledge_detector_store,
        ledge_detector_corner_detector_store,
        transform_store,
        size_store,
        hitbox_store,
        collider_store,
        follow_store,
        death_bound_store,
    ): (
        &mut Entities,
        &mut WriteStorage<LedgeDetector>,
        &mut WriteStorage<LedgeDetectorCornerDetector>,
        &mut WriteStorage<Transform>,
        &mut WriteStorage<Size>,
        &mut WriteStorage<Hitbox>,
        &mut WriteStorage<Collider<CollisionTag>>,
        &mut WriteStorage<Follow>,
        &mut WriteStorage<DeathBound>,
    ),
) -> amethyst::Result<()> {
    if let Some(existing_ledge_detector) = ledge_detector_store.get_mut(entity)
    {
        for corner_entity in existing_ledge_detector.drain_corner_entities() {
            let _ = entities.delete(corner_entity)?;
        }
    }

    let owner_half_size = owner_size.half();
    let collison_tag = CollisionTag {
        labels:        Default::default(),
        collides_with: ledge_detector_data.collides_with,
    };

    let corner_entities = ledge_detector_data
        .corners
        .into_iter()
        .map(|corner| {
            use self::LedgeDetectorCorner as Corner;

            let transform = Transform::default();
            let size = Size::from(corner.size);
            let half_size = size.half();
            let hitbox = Hitbox::from(vec![(&size).into()]);

            let follow_offset = match &corner.corner {
                Corner::TopLeft => (
                    -owner_half_size.w - half_size.w + corner.offset.0,
                    owner_half_size.h + half_size.h + corner.offset.1,
                ),
                Corner::TopRight => (
                    owner_half_size.w + half_size.w + corner.offset.0,
                    owner_half_size.h + half_size.h + corner.offset.1,
                ),
                Corner::BottomLeft => (
                    -owner_half_size.w - half_size.w + corner.offset.0,
                    -owner_half_size.h - half_size.h + corner.offset.1,
                ),
                Corner::BottomRight => (
                    owner_half_size.w + half_size.w + corner.offset.0,
                    -owner_half_size.h - half_size.h + corner.offset.1,
                ),
            };

            entities
                .build_entity()
                .with(transform, transform_store)
                .with(size, size_store)
                .with(hitbox, hitbox_store)
                .with(
                    Follow::new(entity).with_offset(follow_offset),
                    follow_store,
                )
                .with(Collider::new(collison_tag.clone()), collider_store)
                .with(
                    LedgeDetectorCornerDetector::builder()
                        .owner(entity)
                        .corner(corner.corner)
                        .if_touching(corner.if_touching)
                        .build()
                        .unwrap(),
                    ledge_detector_corner_detector_store,
                )
                .with(DeathBound::new(entity), death_bound_store)
                .build()
        })
        .collect();

    ledge_detector_store.insert(entity, LedgeDetector::new(corner_entities))?;
    Ok(())
}
