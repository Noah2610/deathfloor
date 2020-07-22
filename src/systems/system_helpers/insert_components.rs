use super::system_prelude::*;

pub fn insert_components(
    entity: Entity,
    components: EntityComponentsData,
    mut storages: &mut EntityComponentsStorages,
) -> Result<(), amethyst::Error> {
    let EntityComponentsData {
        size,
        velocity,
        gravity,
        max_movement_velocity,
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
    } = components;
    let &mut EntityComponentsStorages {
        entities,
        transform: transform_store,
        size: size_store,
        velocity: velocity_store,
        gravity: gravity_store,
        max_movement_velocity: max_movement_velocity_store,
        base_friction: base_friction_store,
        animation: animation_store,
        animations: animations_store,
        animation_editor: animation_editor_store,
        hitbox: hitbox_store,
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
        collider: collider_store,
        solid: solid_store,
        follow: follow_store,
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
        let owner_half_size = size_opt
            .as_ref()
            .ok_or_else(|| {
                amethyst::Error::from_string(String::from(
                    "LedgeDetector entity needs to have a size",
                ))
            })?
            .half();
        // let solid_tag = solid_store
        //     .get(entity)
        //     .ok_or_else(|| {
        //         amethyst::Error::from_string(String::from(
        //             "LedgeDetector entity needs to have solid collision",
        //         ))
        //     })?
        //     .collision_tag()
        //     .clone();
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
                        -owner_half_size.w - half_size.w - corner.offset.0,
                        owner_half_size.h + half_size.h + corner.offset.1,
                    ),
                    Corner::TopRight => (
                        owner_half_size.w + half_size.w + corner.offset.0,
                        owner_half_size.h + half_size.h + corner.offset.1,
                    ),
                    Corner::BottomLeft => (
                        -owner_half_size.w - half_size.w - corner.offset.0,
                        -owner_half_size.h - half_size.h - corner.offset.1,
                    ),
                    Corner::BottomRight => (
                        owner_half_size.w + half_size.w + corner.offset.0,
                        -owner_half_size.h - half_size.h - corner.offset.1,
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
                    .build()
            })
            .collect();

        ledge_detector_store
            .insert(entity, LedgeDetector::new(corner_entities))?;
    }
    if let Some(size) = size_opt {
        size_store.insert(entity, size)?;
    }

    Ok(())
}
