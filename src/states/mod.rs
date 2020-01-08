mod ingame;
mod startup;

pub mod prelude {
    pub use super::aliases::*;
    pub use super::ingame::Ingame;
    pub use super::startup::Startup;
    pub use super::CustomData;
    pub use super::DispatcherId;
}

pub mod state_prelude {
    pub use amethyst::ecs::{Entity, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use amethyst::renderer::Camera;
    pub use amethyst::{State, StateData, StateEvent, Trans};
    pub use deathframe::amethyst;
    pub use deathframe::custom_game_data::prelude::*;
    pub use deathframe::handles::prelude::*;

    pub use super::prelude::*;
    pub use crate::components::prelude::*;
}

mod aliases {
    use super::{CustomData, DispatcherId};
    use deathframe::custom_game_data::prelude::*;

    pub type GameData<'a, 'b> =
        CustomGameData<'a, 'b, DispatcherId, CustomData>;

    pub type GameDataBuilder<'a, 'b> =
        CustomGameDataBuilder<'a, 'b, DispatcherId, CustomData>;
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum DispatcherId {
    Ingame,
}

#[derive(Default)]
pub struct CustomData;
