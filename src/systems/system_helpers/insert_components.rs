use super::system_prelude::*;
use crate::entity_config::prelude::*;

use self::EntityConfigComponent as Comp;

pub fn insert_components(
    entity: Entity,
    components: EntityConfigComponents,
    storages: &mut EntityConfigComponentsStorages,
) -> amethyst::Result<()> {
    let size_opt = storages.size.get(entity).cloned();

    for component in components.components {
        match component {
            Comp::Velocity(velocity) => match velocity {
                Some(velocity) => {
                    storages.velocity.insert(entity, velocity)?;
                }
                None => {
                    storages.velocity.remove(entity);
                }
            },
            Comp::Gravity(gravity) => match gravity {
                Some(gravity) => {
                    storages.gravity.insert(entity, gravity)?;
                }
                None => {
                    storages.gravity.remove(entity);
                }
            },
            Comp::MaxMovementVelocity(max_movement_velocity) => {
                match max_movement_velocity {
                    Some(max_movement_velocity) => {
                        storages
                            .max_movement_velocity
                            .insert(entity, max_movement_velocity)?;
                    }
                    None => {
                        storages.max_movement_velocity.remove(entity);
                    }
                }
            }
            Comp::MovementAcceleration(movement_acceleration) => {
                match movement_acceleration {
                    Some(movement_acceleration) => {
                        storages
                            .movement_acceleration
                            .insert(entity, movement_acceleration)?;
                    }
                    None => {
                        storages.movement_acceleration.remove(entity);
                    }
                }
            }
            Comp::BaseFriction(base_friction) => match base_friction {
                Some(base_friction) => {
                    storages.base_friction.insert(entity, base_friction)?;
                }
                None => {
                    storages.base_friction.remove(entity);
                }
            },
            Comp::Animation(animation) => match animation {
                Some(mut animation) => {
                    animation.play_cycle();
                    storages.animation.insert(entity, animation)?;
                }
                None => {
                    storages.animation.remove(entity);
                }
            },
            Comp::Animations(animations) => match animations {
                Some(animations) => {
                    storages.animations.insert(entity, animations)?;
                    storages
                        .animation_editor
                        .insert(entity, AnimationEditor::default())?;
                }
                None => {
                    storages.animations.remove(entity);
                }
            },
            Comp::Hitbox(hitbox_config) => match hitbox_config {
                Some(hitbox_config) => {
                    let hitbox = match hitbox_config {
                        HitboxConfig::Size => {
                            if let Some(size) = size_opt.as_ref() {
                                Hitbox::new().with_rect(Rect::from(size))
                            } else {
                                panic!(
                                    "Cannot create `Hitbox` with \
                                     `HitboxConfig::Size` because entity has \
                                     no size!"
                                )
                            }
                        }
                        HitboxConfig::Custom(rects) => {
                            Hitbox::new().with_rects(rects.clone())
                        }
                    };
                    storages.hitbox.insert(entity, hitbox)?;
                    // TODO this doesn't seem right...
                    storages
                        .jumppad_affected
                        .insert(entity, JumppadAffected::default())?;
                }
                None => {
                    storages.hitbox.remove(entity);
                }
            },
            Comp::Walker(walker) => match walker {
                Some(walker) => {
                    storages.walker.insert(entity, walker)?;
                }
                None => {
                    storages.walker.remove(entity);
                }
            },
            Comp::Jumppad(jumppad) => match jumppad {
                Some(jumppad) => {
                    storages.jumppad.insert(entity, jumppad)?;
                }
                None => {
                    storages.jumppad.remove(entity);
                }
            },
            Comp::ScaleOnce(scale_once) => match scale_once {
                Some(scale_once) => {
                    storages.scale_once.insert(entity, scale_once)?;
                }
                None => {
                    storages.scale_once.remove(entity);
                }
            },
            Comp::Health(health) => match health {
                Some(health) => {
                    storages.health.insert(entity, health)?;
                    storages
                        .health_action_queue
                        .insert(entity, HealthActionQueue::default())?;
                }
                None => {
                    storages.health.remove(entity);
                }
            },
            Comp::HealthDisplay(health_display) => match health_display {
                Some(health_display) => {
                    storages.health_display.insert(entity, health_display)?;
                }
                None => {
                    storages.health_display.remove(entity);
                }
            },
            Comp::DealsDamage(deals_damage) => match deals_damage {
                Some(deals_damage) => {
                    storages.deals_damage.insert(entity, deals_damage)?;
                }
                None => {
                    storages.deals_damage.remove(entity);
                }
            },
            Comp::TakesDamage(takes_damage) => match takes_damage {
                Some(takes_damage) => {
                    storages.takes_damage.insert(entity, takes_damage)?;
                }
                None => {
                    storages.takes_damage.remove(entity);
                }
            },
            Comp::Bullet(bullet) => match bullet {
                Some(bullet) => {
                    storages.bullet.insert(entity, bullet)?;
                }
                None => {
                    storages.bullet.remove(entity);
                }
            },
            Comp::LedgeDetector(ledge_detector_data) => {
                match ledge_detector_data {
                    Some(ledge_detector_data) => {
                        insert_ledge_detector(
                            entity,
                            ledge_detector_data,
                            size_opt.as_ref().ok_or_else(|| {
                                amethyst::Error::from_string(String::from(
                                    "LedgeDetector entity needs to have a size",
                                ))
                            })?,
                            (
                                &mut storages.entities,
                                &mut storages.ledge_detector,
                                &mut storages.ledge_detector_corner_detector,
                                &mut storages.transform,
                                &mut storages.size,
                                &mut storages.hitbox,
                                &mut storages.collider,
                                &mut storages.follow,
                                &mut storages.death_bound,
                            ),
                        )?;
                    }
                    None => {
                        if let Some(mut ledge_detector) =
                            storages.ledge_detector.remove(entity)
                        {
                            for corner_entity in
                                ledge_detector.drain_corner_entities()
                            {
                                storages
                                    .entities
                                    .delete(corner_entity)
                                    .unwrap();
                            }
                        }
                    }
                }
            }
            Comp::DeathOnContact(death_on_contact) => match death_on_contact {
                Some(death_on_contact) => {
                    storages
                        .death_on_contact
                        .insert(entity, death_on_contact)?;
                }
                None => {
                    storages.death_on_contact.remove(entity);
                }
            },
            Comp::DeathAfterDelay(death_after_delay) => match death_after_delay
            {
                Some(death_after_delay) => {
                    storages
                        .death_after_delay
                        .insert(entity, death_after_delay)?;
                }
                None => {
                    storages.death_after_delay.remove(entity);
                }
            },
            Comp::Interactable(interactable) => match interactable {
                Some(interactable) => {
                    storages.interactable.insert(entity, interactable)?;
                }
                None => {
                    storages.interactable.remove(entity);
                }
            },
            Comp::Facing(facing) => match facing {
                Some(facing) => {
                    storages.facing.insert(entity, facing)?;
                }
                None => {
                    storages.facing.remove(entity);
                }
            },
            Comp::Jumper(jumper) => match jumper {
                Some(jumper) => {
                    storages.jumper.insert(entity, jumper)?;
                }
                None => {
                    storages.jumper.remove(entity);
                }
            },
            Comp::WallJumper(wall_jumper) => match wall_jumper {
                Some(wall_jumper) => {
                    storages.wall_jumper.insert(entity, wall_jumper)?;
                }
                None => {
                    storages.wall_jumper.remove(entity);
                }
            },
            Comp::WallSlider(wall_slider) => match wall_slider {
                Some(wall_slider) => {
                    storages.wall_slider.insert(entity, wall_slider)?;
                }
                None => {
                    storages.wall_slider.remove(entity);
                }
            },
            Comp::Shooter(shooter) => match shooter {
                Some(shooter) => {
                    storages.shooter.insert(entity, shooter)?;
                }
                None => {
                    storages.shooter.remove(entity);
                }
            },
            Comp::KillVelocityMin(kill_velocity_min) => match kill_velocity_min
            {
                Some(kill_velocity_min) => {
                    storages
                        .kill_velocity_min
                        .insert(entity, kill_velocity_min)?;
                }
                None => {
                    storages.kill_velocity_min.remove(entity);
                }
            },
            Comp::SolidPusher(solid_pusher) => match solid_pusher {
                Some(solid_pusher) => {
                    storages.solid_pusher.insert(entity, solid_pusher)?;
                }
                None => {
                    storages.solid_pusher.remove(entity);
                }
            },
            Comp::SolidPushable(solid_pushable) => match solid_pushable {
                Some(solid_pushable) => {
                    storages.solid_pushable.insert(entity, solid_pushable)?;
                }
                None => {
                    storages.solid_pushable.remove(entity);
                }
            },
            Comp::NonPreciseMovement(non_precise_movement) => {
                match non_precise_movement {
                    Some(non_precise_movement) => {
                        storages
                            .non_precise_movement
                            .insert(entity, non_precise_movement)?;
                    }
                    None => {
                        storages.non_precise_movement.remove(entity);
                    }
                }
            }
            Comp::VariableRegister(variable_register) => {
                match variable_register {
                    Some(variable_register) => {
                        storages
                            .variable_register
                            .insert(entity, variable_register)?;
                        storages.update_variable_register.insert(
                            entity,
                            UpdateVariableRegister::default(),
                        )?;
                    }
                    None => {
                        storages.variable_register.remove(entity);
                        storages.update_variable_register.remove(entity);
                    }
                }
            }
            Comp::Loader(loader) => match loader {
                Some(loader) => {
                    storages.loader.insert(entity, loader)?;
                }
                None => {
                    storages.loader.remove(entity);
                }
            },
            Comp::Loadable(loadable) => match loadable {
                Some(loadable) => {
                    storages.loadable.insert(entity, loadable)?;
                }
                None => {
                    storages.loadable.remove(entity);
                }
            },
            Comp::Unloaded(unloaded) => match unloaded {
                Some(unloaded) => {
                    storages.unloaded.insert(entity, unloaded)?;
                }
                None => {
                    storages.unloaded.remove(entity);
                }
            },
        }
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
