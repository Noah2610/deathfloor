mod ingame_bindings;

pub mod prelude {
    pub use super::ingame_bindings::{
        IngameActionBinding,
        IngameAxisBinding,
        IngameBindings,
    };
    pub use IngameActionBinding::*;
    pub use IngameAxisBinding::*;
}

pub use ingame_bindings::IngameBindings;

use crate::helpers::resource;
use amethyst::input::InputBundle;
use deathframe::amethyst;

pub fn ingame_input_bundle(
) -> amethyst::Result<InputBundle<ingame_bindings::IngameBindings>> {
    Ok(InputBundle::<ingame_bindings::IngameBindings>::new()
        .with_bindings_from_file(resource("config/ingame_bindings.ron"))?)
}
