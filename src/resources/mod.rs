pub mod prelude {
    pub use super::bullet_creator::{BulletComponents, BulletCreator};
    pub use deathframe::resources::prelude::*;
}

mod bullet_creator;

use amethyst::ecs::World;
use deathframe::amethyst;

pub fn insert_resources(world: &mut World) -> amethyst::Result<()> {
    use crate::input::prelude::PausedBindings;
    use crate::settings::Settings;
    use deathframe::resources::input_manager::InputManager;
    use prelude::*;

    world.insert(SpriteSheetHandles::default());
    world.insert(Settings::load()?);
    world.insert(InputManager::<PausedBindings>::default());

    Ok(())
}
