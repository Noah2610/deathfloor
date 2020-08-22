pub(super) mod prelude {
    pub(in super::super) use super::base_entity;
    pub(in super::super) use super::edit_entity_with_entity_config;
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

use crate::systems::system_helpers::prelude::add_entity_config;
use amethyst::ecs::SystemData;
use deathframe::resources::SpriteSheetHandles;
use prelude::*;
use std::path::PathBuf;

pub(super) fn base_entity<'a, T>(
    world: &'a mut World,
    propful: &T,
) -> amethyst::Result<EntityBuilder<'a>>
where
    T: Propful,
{
    let mut transform: Transform = propful.pos().into();
    transform.set_translation_z(propful.z_or_default());

    let scale_x = propful
        .props()
        .get("scale_x")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);
    let scale_y = propful
        .props()
        .get("scale_y")
        .and_then(|v| v.as_f64())
        .map(|v| v as f32);

    let scale = transform.scale_mut();
    scale_x.map(|x| scale.x = x);
    scale_y.map(|y| scale.y = y);

    Ok(world
        .create_entity()
        .with(transform)
        .with(ScaleOnce::default())
        .with(Transparent))
}

pub(super) fn get_sprite_render<P>(
    world: &mut World,
    spritesheet_path: P,
    sprite_number: usize,
) -> amethyst::Result<SpriteRender>
where
    P: Into<PathBuf>,
{
    let handle = world
        .write_resource::<SpriteSheetHandles<PathBuf>>()
        .get_or_load(resource(spritesheet_path), world);
    Ok(SpriteRender {
        sprite_sheet:  handle,
        sprite_number: sprite_number,
    })
}

/// Edits `Entity` with the given `EntityConfig`.
pub(super) fn edit_entity_with_entity_config(
    world: &mut World,
    entity: Entity,
    entity_config: EntityConfig,
    variant_prop: Option<String>,
) -> amethyst::Result<()> {
    add_entity_config(
        entity,
        entity_config,
        variant_prop,
        SystemData::fetch(world),
    )
}
