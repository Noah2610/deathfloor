mod animation_editor;
mod bullet;
mod enemy;
mod entity_config_register;
mod event_listener;
mod health_display;
mod jumper;
mod jumppad;
mod jumppad_affected;
mod ledge_detector;
mod max_movement_velocity;
mod movable;
mod physics_data;
mod player;
mod shooter;
mod walker;
mod wall_jumper;
mod wall_slider;

pub mod prelude {
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;

    pub use super::animation_editor::prelude::*;
    pub use super::bullet::Bullet;
    pub use super::enemy::prelude::*;
    pub use super::entity_config_register::{
        EntityConfigRegister,
        EntityConfigRegisterAction,
    };
    pub use super::event_listener::prelude::*;
    pub use super::health_display::prelude::*;
    pub use super::jumper::Jumper;
    pub use super::jumppad::Jumppad;
    pub use super::jumppad_affected::JumppadAffected;
    pub use super::ledge_detector::prelude::*;
    pub use super::max_movement_velocity::MaxMovementVelocity;
    pub use super::movable::prelude::*;
    pub use super::physics_data::PhysicsData;
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
