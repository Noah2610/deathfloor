mod control_player;
mod control_player_jump;
mod control_player_kill_velocity;
mod control_player_shoot;
mod display_health;
mod events_actions;
mod handle_animation_editors;
mod handle_animations;
mod handle_death_after_delay;
mod handle_death_bound;
mod handle_death_on_contact;
mod handle_dying_entities;
mod handle_interactable;
mod handle_jumppad_affected;
mod handle_ledge_detector;
mod handle_level_select;
mod handle_locked_to_path;
mod handle_movables;
mod handle_scales;
mod handle_walkers;
mod play_death_animation_before_deletion;
mod update_entity_configs;

pub mod prelude {
    pub use deathframe::amethyst::utils::ortho_camera::CameraOrthoSystem;
    pub use deathframe::systems::prelude::*;

    pub use super::control_player::ControlPlayerSystem;
    pub use super::control_player_jump::ControlPlayerJumpSystem;
    pub use super::control_player_kill_velocity::ControlPlayerKillVelocitySystem;
    pub use super::control_player_shoot::ControlPlayerShootSystem;
    pub use super::display_health::DisplayHealthSystem;
    pub use super::events_actions::EventsActionsBundle;
    pub use super::handle_animation_editors::HandleAnimationEditorsSystem;
    pub use super::handle_animations::HandleAnimationsSystem;
    pub use super::handle_death_after_delay::HandleDeathAfterDelaySystem;
    pub use super::handle_death_bound::HandleDeathBoundSystem;
    pub use super::handle_death_on_contact::HandleDeathOnContactSystem;
    pub use super::handle_dying_entities::HandleDyingEntitiesSystem;
    pub use super::handle_interactable::HandleInteractableSystem;
    pub use super::handle_jumppad_affected::HandleJumppadAffectedSystem;
    pub use super::handle_ledge_detector::HandleLedgeDetectorSystem;
    pub use super::handle_level_select::HandleLevelSelectSystem;
    pub use super::handle_locked_to_path::HandleLockedToPathSystem;
    pub use super::handle_movables::HandleMovablesSystem;
    pub use super::handle_scales::HandleScalesSystem;
    pub use super::handle_walkers::HandleWalkersSystem;
    pub use super::play_death_animation_before_deletion::PlayDeathAnimationBeforeDeletionSystem;
    pub use super::update_entity_configs::UpdateEntityConfigsSystem;
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
