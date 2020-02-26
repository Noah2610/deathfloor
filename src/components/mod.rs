mod can_jump;
mod max_movement_velocity;
mod movable;
mod movement_data;
mod player;

pub mod prelude {
    pub use amethyst::renderer::SpriteRender;
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;

    pub use super::can_jump::CanJump;
    pub use super::max_movement_velocity::MaxMovementVelocity;
    pub use super::movable::{Movable, MoveAction};
    pub use super::movement_data::MovementData;
    pub use super::player::Player;
    pub use crate::collision_tag::{CollisionTag, SolidTag};
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
