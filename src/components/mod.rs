pub mod animation_editor;
pub mod bullet;
pub mod death_bound;
pub mod enemy;
pub mod entity_config_register;
pub mod event_listener;
pub mod health_display;
pub mod jumper;
pub mod jumppad;
pub mod jumppad_affected;
pub mod ledge_detector;
pub mod max_movement_velocity;
pub mod movable;
pub mod physics_data;
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
    pub use super::death_bound::DeathBound;
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
