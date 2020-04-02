mod bullet_hit;
mod control_player;
mod control_player_jump;
mod control_player_shoot;
mod create_bullets;
#[cfg(feature = "debug")]
mod debug;
mod delete_bullets;
mod event_handlers;
mod handle_animations;
mod handle_jumppad_affected;
mod handle_movables;
mod handle_scales;
mod handle_walkers;

pub mod prelude {
    pub use deathframe::amethyst::utils::ortho_camera::CameraOrthoSystem;
    pub use deathframe::systems::prelude::*;

    pub use super::bullet_hit::BulletHitSystem;
    pub use super::control_player::ControlPlayerSystem;
    pub use super::control_player_jump::ControlPlayerJumpSystem;
    pub use super::control_player_shoot::ControlPlayerShootSystem;
    pub use super::create_bullets::CreateBulletsSystem;
    #[cfg(feature = "debug")]
    pub use super::debug::DebugSystem;
    pub use super::delete_bullets::DeleteBulletsSystem;
    pub use super::event_handlers::EventHandlersBundle;
    pub use super::handle_animations::HandleAnimationsSystem;
    pub use super::handle_jumppad_affected::HandleJumppadAffectedSystem;
    pub use super::handle_movables::HandleMovablesSystem;
    pub use super::handle_scales::HandleScalesSystem;
    pub use super::handle_walkers::HandleWalkersSystem;
}

mod system_prelude {
    pub use deathframe::amethyst;
    pub use deathframe::physics::query::prelude::*;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::animation_key::AnimationKey;
    pub use crate::collision_tag::{CollisionTag, SolidTag};
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;

    pub use super::system_helpers::*;
}

pub mod system_helpers {
    use super::system_prelude::*;
    use crate::settings::prelude::*;

    pub fn insert_components(
        entity: Entity,
        components: EnemyComponentsData,
        mut storages: &mut EnemyComponentsStorages,
    ) -> Result<(), amethyst::ecs::error::Error> {
        let EnemyComponentsData {
            size,
            gravity,
            max_movement_velocity,
            base_friction,
            animations,
            hitbox,
            walker,
            jumppad,
            scale_once,
        } = components;
        let &mut EnemyComponentsStorages {
            size: size_store,
            gravity: gravity_store,
            max_movement_velocity: max_movement_velocity_store,
            base_friction: base_friction_store,
            animations: animations_store,
            hitbox: hitbox_store,
            walker: walker_store,
            jumppad: jumppad_store,
            jumppad_affected: jumppad_affected_store,
            scale_once: scale_once_store,
        } = &mut storages;

        let size_opt = size.or_else(|| size_store.get(entity).cloned());

        if let Some(gravity) = gravity {
            gravity_store.insert(entity, gravity)?;
        }
        if let Some(max_movement_velocity) = max_movement_velocity {
            max_movement_velocity_store
                .insert(entity, max_movement_velocity)?;
        }
        if let Some(base_friction) = base_friction {
            base_friction_store.insert(entity, base_friction)?;
        }
        if let Some(animations) = animations {
            animations_store.insert(entity, animations)?;
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
            jumppad_affected_store
                .insert(entity, JumppadAffected::default())?;
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
        if let Some(size) = size_opt {
            size_store.insert(entity, size)?;
        }

        Ok(())
    }
}
