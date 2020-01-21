pub mod prelude {
    pub use super::settings::SettingsRes;
    pub use deathframe::handles::SpriteSheetHandles;
}

mod settings;

use amethyst::ecs::World;
use deathframe::amethyst;

pub fn insert_resources(world: &mut World) -> amethyst::Result<()> {
    use crate::settings::Settings;
    use prelude::*;

    // NOTE: Settings resource is added in init::init_game
    // world.insert(SettingsRes::new(Settings::load()?));

    world.insert(SpriteSheetHandles::default());

    Ok(())
}
