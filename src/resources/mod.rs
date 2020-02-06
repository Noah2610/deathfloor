pub mod prelude {
    pub use super::settings::SettingsRes;
    pub use deathframe::resources::prelude::*;
}

mod settings;

use amethyst::ecs::World;
use deathframe::amethyst;

pub fn insert_resources(world: &mut World) -> amethyst::Result<()> {
    use crate::settings::Settings;
    use prelude::*;

    world.insert(SpriteSheetHandles::default());
    world.insert(SettingsRes::new(Settings::load()?));

    Ok(())
}
