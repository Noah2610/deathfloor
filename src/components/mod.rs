mod movement_data;
mod player;

pub mod prelude {
    pub use amethyst::renderer::SpriteRender;
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;

    pub use super::movement_data::MovementData;
    pub use super::player::Player;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
