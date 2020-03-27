mod ingame;
mod load_ingame;
mod paused;
mod startup;

pub mod prelude {
    pub use super::aliases::*;
    pub use super::ingame::Ingame;
    pub use super::load_ingame::LoadIngame;
    pub use super::paused::Paused;
    pub use super::startup::Startup;
    pub use super::CustomData;
    pub use crate::dispatcher_id::DispatcherId;
}

pub mod state_prelude {
    pub use amethyst::ecs::{Entity, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use amethyst::renderer::Camera;
    pub use amethyst::{State, StateData, StateEvent, Trans};
    pub use deathframe::amethyst;
    pub use deathframe::core::custom_game_data::prelude::*;

    pub use super::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::resources::prelude::*;
}

mod aliases {
    use super::CustomData;
    use crate::dispatcher_id::DispatcherId;
    use deathframe::core::custom_game_data::prelude::*;

    pub type GameData<'a, 'b> =
        CustomGameData<'a, 'b, DispatcherId, CustomData>;

    pub type GameDataBuilder<'a, 'b> =
        CustomGameDataBuilder<'a, 'b, DispatcherId, CustomData>;
}

#[derive(Default)]
pub struct CustomData;
