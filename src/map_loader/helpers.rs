pub(super) mod prelude {
    pub(in super::super) use super::get_sprite_render;
    pub use crate::animation_key::AnimationKey;
    pub use crate::components::prelude::*;
    pub use crate::helpers::resource;
    pub use crate::map_loader::map_data::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use amethyst::ecs::{Entity, EntityBuilder, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use deathframe::amethyst;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::resources::SpriteSheetHandles;
}

use deathframe::resources::SpriteSheetHandles;
use prelude::*;
use std::path::{Path, PathBuf};

pub(super) fn get_sprite_render<P>(
    world: &mut World,
    spritesheet_path: P,
    sprite_number: usize,
) -> amethyst::Result<SpriteRender>
where
    P: AsRef<Path>,
{
    let handle = world
        .write_resource::<SpriteSheetHandles<PathBuf>>()
        .get_or_load(resource(spritesheet_path), world);
    Ok(SpriteRender {
        sprite_sheet:  handle,
        sprite_number: sprite_number,
    })
}
