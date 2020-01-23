pub(super) mod prelude {
    pub(in super::super) use super::get_sprite_render;
    pub use crate::components::prelude::*;
    pub use crate::helpers::resource;
    pub use crate::map_loader::map_data::prelude::*;
    pub use crate::resources::prelude::*;
    pub use crate::settings::prelude::*;
    pub use amethyst::ecs::{Entity, EntityBuilder, World, WorldExt};
    pub use amethyst::prelude::Builder;
    pub use deathframe::geo::Vector;
    pub use deathframe::{amethyst, specs_physics};
    pub use specs_physics::nalgebra::geometry::Isometry2;
    pub use specs_physics::EntityBuilderExt;
}

use prelude::*;

pub(super) fn get_sprite_render<S>(
    world: &mut World,
    spritesheet_path: S,
    sprite_number: usize,
) -> amethyst::Result<SpriteRender>
where
    S: Into<String>,
{
    let handle = world
        .write_resource::<SpriteSheetHandles>()
        .get_or_load(resource(spritesheet_path), world);
    Ok(SpriteRender {
        sprite_sheet:  handle,
        sprite_number: sprite_number,
    })
}
