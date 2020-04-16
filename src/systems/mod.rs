mod bullet_hit;
mod control_player;
mod control_player_jump;
mod control_player_shoot;
mod create_bullets;
mod delete_bullets;
mod display_health;
mod event_handlers;
mod handle_animation_editors;
mod handle_animations;
mod handle_entity_lifecycle;
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
    pub use super::delete_bullets::DeleteBulletsSystem;
    pub use super::display_health::DisplayHealthSystem;
    pub use super::event_handlers::EventHandlersBundle;
    pub use super::handle_animation_editors::HandleAnimationEditorsSystem;
    pub use super::handle_animations::HandleAnimationsSystem;
    pub use super::handle_entity_lifecycle::HandleEntityLifecycleSystem;
    pub use super::handle_jumppad_affected::HandleJumppadAffectedSystem;
    pub use super::handle_movables::HandleMovablesSystem;
    pub use super::handle_scales::HandleScalesSystem;
    pub use super::handle_walkers::HandleWalkersSystem;
}

pub mod system_prelude {
    pub use deathframe::amethyst;
    pub use deathframe::physics::query::prelude::*;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::animation_key::AnimationKey;
    pub use crate::collision_tag::{CollisionTag, SolidTag};
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;

    pub use super::system_helpers::prelude::*;
}

pub mod system_helpers;
