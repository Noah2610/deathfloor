pub(super) mod prelude {
    pub(in super::super) use super::base_object_entity;
    pub(in super::super) use super::get_sprite_render;
    pub(in super::super) use crate::components::prelude::*;
    pub(in super::super) use crate::helpers::resource;
    pub(in super::super) use crate::map_loader::map_data::prelude::*;
    pub(in super::super) use crate::resources::prelude::*;
    pub(in super::super) use amethyst::ecs::{
        Entity,
        EntityBuilder,
        World,
        WorldExt,
    };
    pub(in super::super) use amethyst::prelude::Builder;
    pub(in super::super) use deathframe::geo::Vector;
    pub(in super::super) use deathframe::{amethyst, specs_physics};
    pub(in super::super) use specs_physics::ncollide::shape::{
        Cuboid,
        ShapeHandle,
    };
    pub(in super::super) use specs_physics::nphysics::object::{
        Body,
        BodyStatus,
        ColliderDesc,
        RigidBodyDesc,
    };
    pub(in super::super) use specs_physics::EntityBuilderExt;
}

use deathframe::handles::SpriteSheetHandles;
use prelude::*;

/// Adds base components to object entity.
/// Components include:
///     - `Transform`
///     - `Size`
///     - `ScaleOnce`
///     - `Transparent`
pub(super) fn base_object_entity<'a>(
    world: &'a mut World,
    object: &ObjectData,
) -> amethyst::Result<EntityBuilder<'a>> {
    const DEFAULT_Z: f32 = 1.0;

    let mut transform: Transform = object.pos.into();
    transform.set_translation_z(object.z_or(DEFAULT_Z));

    let size: Size = object.size.into();

    let entity = world
        .create_entity()
        .with(transform)
        .with(size.clone())
        .with(ScaleOnce::default())
        .with(Transparent);

    Ok(entity)
}

pub(super) fn get_sprite_render(
    world: &mut World,
    spritesheet_path: &str,
    sprite_number: usize,
) -> amethyst::Result<SpriteRender> {
    let handle = world
        .write_resource::<SpriteSheetHandles>()
        .get_or_load(resource(spritesheet_path), world);
    Ok(SpriteRender {
        sprite_sheet:  handle,
        sprite_number: sprite_number,
    })
}
