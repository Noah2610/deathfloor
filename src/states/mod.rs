pub mod ingame;
pub mod level_select;
pub mod load_ingame;
pub mod main_menu;
pub mod paused;
pub mod startup;

pub mod prelude {
    pub use super::aliases::*;
    pub use super::ingame::Ingame;
    pub use super::level_select::LevelSelect;
    pub use super::load_ingame::LoadIngame;
    pub use super::main_menu::MainMenu;
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

    pub const BGM: super::state_prelude::SongType =
        super::state_prelude::SongType::Menu;
}

#[derive(Default)]
pub struct CustomData;
