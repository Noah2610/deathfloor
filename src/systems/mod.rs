mod control_player;
mod control_player_jump;
#[cfg(feature = "debug")]
mod debug;
mod handle_animations;
mod handle_movables;

pub mod prelude {
    pub use deathframe::amethyst::utils::ortho_camera::CameraOrthoSystem;
    pub use deathframe::systems::prelude::*;

    pub use super::control_player::ControlPlayerSystem;
    pub use super::control_player_jump::ControlPlayerJumpSystem;
    #[cfg(feature = "debug")]
    pub use super::debug::DebugSystem;
    pub use super::handle_animations::HandleAnimationsSystem;
    pub use super::handle_movables::HandleMovablesSystem;
}

mod system_prelude {
    pub use deathframe::amethyst;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::animation_key::AnimationKey;
    pub use crate::collision_tag::{CollisionTag, SolidTag};
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
}
