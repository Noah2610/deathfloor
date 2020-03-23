mod bullet;
mod enemy;
mod jumper;
mod jumppad;
mod jumppad_affected;
mod max_movement_velocity;
mod movable;
mod physics_data;
mod player;
mod shooter;
mod wall_jumper;
mod wall_slider;

pub mod prelude {
    pub use amethyst::renderer::SpriteRender;
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;

    pub use super::bullet::Bullet;
    pub use super::enemy::{Enemy, EnemyType};
    pub use super::jumper::Jumper;
    pub use super::jumppad::Jumppad;
    pub use super::jumppad_affected::JumppadAffected;
    pub use super::max_movement_velocity::MaxMovementVelocity;
    pub use super::movable::{Movable, MoveAction};
    pub use super::physics_data::PhysicsData;
    pub use super::player::Player;
    pub use super::shooter::Shooter;
    pub use super::wall_jumper::WallJumper;
    pub use super::wall_slider::WallSlider;
    pub use crate::collision_tag::{CollisionTag, SolidTag};
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
