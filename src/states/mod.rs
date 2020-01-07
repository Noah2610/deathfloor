mod startup;

pub mod prelude {
    pub use super::startup::Startup;
    pub use super::CustomData;
    pub use super::DispatcherId;
    pub use super::{GameData, GameDataBuilder};
}

pub mod state_prelude {
    pub use deathframe::amethyst::ecs::{World, WorldExt};
    pub use deathframe::amethyst::{State, StateData, StateEvent, Trans};
    pub use deathframe::custom_game_data::prelude::*;

    pub use super::prelude::*;
}

#[derive(Hash, PartialEq, Eq, Debug)]
pub enum DispatcherId {
    Startup,
}

pub type GameData<'a, 'b> = deathframe::custom_game_data::CustomGameData<
    'a,
    'b,
    DispatcherId,
    CustomData,
>;

pub type GameDataBuilder<'a, 'b> =
    deathframe::custom_game_data::CustomGameDataBuilder<
        'a,
        'b,
        DispatcherId,
        CustomData,
    >;

#[derive(Default)]
pub struct CustomData;
