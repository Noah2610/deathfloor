use super::system_prelude::*;

pub fn insert_components(
    entity: Entity,
    components: EntityComponentsData,
    mut storages: &mut EntityComponentsStorages,
) -> Result<(), amethyst::ecs::error::Error> {
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
    } = components;
    let &mut EntityComponentsStorages {
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
    if let Some(size) = size_opt {
        size_store.insert(entity, size)?;
    }

    Ok(())
}
