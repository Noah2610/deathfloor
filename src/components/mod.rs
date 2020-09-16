pub mod animation_editor;
pub mod bullet;
pub mod can_interact;
pub mod death_after_delay;
pub mod death_bound;
pub mod death_on_contact;
pub mod enemy;
pub mod entity_config_register;
pub mod events_register;
pub mod facing;
pub mod health_display;
pub mod interactable;
pub mod jumper;
pub mod jumppad;
pub mod jumppad_affected;
pub mod ledge_detector;
pub mod locked_to_path;
pub mod max_movement_velocity;
pub mod movable;
pub mod movement_acceleration;
pub mod player;
pub mod shooter;
pub mod walker;
pub mod wall_jumper;
pub mod wall_slider;

pub mod prelude {
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;

    pub use super::animation_editor::prelude::*;
    pub use super::bullet::Bullet;
    pub use super::can_interact::CanInteract;
    pub use super::death_after_delay::DeathAfterDelay;
    pub use super::death_bound::DeathBound;
    pub use super::death_on_contact::DeathOnContact;
    pub use super::enemy::prelude::*;
    pub use super::entity_config_register::{
        EntityConfigRegister,
        EntityConfigRegisterAction,
    };
    pub use super::events_register::prelude::*;
    pub use super::facing::Facing;
    pub use super::health_display::prelude::*;
    pub use super::interactable::{Interactable, InteractableAction};
    pub use super::jumper::Jumper;
    pub use super::jumppad::Jumppad;
    pub use super::jumppad_affected::JumppadAffected;
    pub use super::ledge_detector::prelude::*;
    pub use super::locked_to_path::LockedToPath;
    pub use super::max_movement_velocity::MaxMovementVelocity;
    pub use super::movable::prelude::*;
    pub use super::movement_acceleration::MovementAcceleration;
    pub use super::player::Player;
    pub use super::shooter::Shooter;
    pub use super::walker::Walker;
    pub use super::wall_jumper::WallJumper;
    pub use super::wall_slider::WallSlider;
    pub use crate::collision_tag::prelude::*;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
