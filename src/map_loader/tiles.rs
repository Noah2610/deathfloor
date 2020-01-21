use super::map_data::prelude::*;
use crate::components::prelude::*;
use crate::helpers::resource;
use amethyst::ecs::{World, WorldExt};
use amethyst::prelude::Builder;
use deathframe::geo::Vector;
use deathframe::handles::SpriteSheetHandles;
use deathframe::{amethyst, specs_physics};
use specs_physics::ncollide::shape::{Cuboid, ShapeHandle};
use specs_physics::nphysics::object::{
    BodyStatus,
    ColliderDesc,
    RigidBodyDesc,
};
use specs_physics::EntityBuilderExt;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: &TilesData,
    tile_size: SizeData,
) -> amethyst::Result<()> {
    const DEFAULT_Z: f32 = 0.0;

    let size: Size = tile_size.into();

    for tile in tiles {
        let mut transform: Transform = tile.pos.into();
        transform.set_translation_z(tile.z_or(DEFAULT_Z));
        let pos = {
            let translation = transform.translation();
            Vector::new(translation.x, translation.y)
        };

        let sprite_render_opt = {
            let spritesheet_path =
                resource(format!("spritesheets/tiles/{}", tile.ts));
            let spritesheet_handle = world
                .write_resource::<SpriteSheetHandles>()
                .get_or_load(&spritesheet_path, world);
            Some(SpriteRender {
                sprite_sheet:  spritesheet_handle.clone(),
                sprite_number: tile.id,
            })
        };

        let mut entity = world
            .create_entity()
            .with(transform)
            .with(size.clone())
            .with(ScaleOnce::default())
            .with(Transparent);

        if tile.is_solid() {
            // entity = entity.with(Solid::new(SolidTag::Tile));

            let body = RigidBodyDesc::<f32>::new()
                .translation(pos)
                .gravity_enabled(false)
                .status(BodyStatus::Static)
                .build();
            let shape = ShapeHandle::new(Cuboid::new(Vector::new(
                size.w * 0.5,
                size.h * 0.5,
            )));
            let collider = ColliderDesc::new(shape);

            entity = entity
                .with_body::<f32, _>(body)
                .with_collider::<f32>(&collider);
        }

        if let Some(sprite_render) = sprite_render_opt {
            entity = entity.with(sprite_render);
        }

        entity.build();
    }

    Ok(())
}
