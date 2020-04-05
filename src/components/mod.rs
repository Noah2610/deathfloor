mod animation_editor;
mod bullet;
mod damage;
mod enemy;
mod event_listener;
mod health;
mod jumper;
mod jumppad;
mod jumppad_affected;
mod lifecycle;
mod max_movement_velocity;
mod movable;
mod physics_data;
mod player;
mod shooter;
mod walker;
mod wall_jumper;
mod wall_slider;

pub mod prelude {
    pub use amethyst::renderer::SpriteRender;
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;

    pub use super::animation_editor::prelude::*;
    pub use super::bullet::Bullet;
    pub use super::component_helpers::prelude::*;
    pub use super::damage::prelude::*;
    pub use super::enemy::prelude::*;
    pub use super::event_listener::prelude::*;
    pub use super::health::prelude::*;
    pub use super::jumper::Jumper;
    pub use super::jumppad::Jumppad;
    pub use super::jumppad_affected::JumppadAffected;
    pub use super::lifecycle::prelude::*;
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

pub mod component_helpers;
