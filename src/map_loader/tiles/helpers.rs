pub(super) mod prelude {
    pub(in super::super) use super::base_tile_entity;
    pub(in super::super) use super::edit_entity_with_tile_settings;
    pub use crate::map_loader::helpers::prelude::*;
}

use prelude::*;

/// Adds components depending on given `TileSettings` to `EntityBuilder`.
pub(super) fn edit_entity_with_tile_settings<'a>(
    mut entity: EntityBuilder<'a>,
    tile_settings: &TileSettings,
    size: &Size,
) -> EntityBuilder<'a> {
    // HITBOX
    if let Some(hitbox_type) = &tile_settings.hitbox {
        let hitbox = match hitbox_type {
            HitboxConfig::Size => Hitbox::new().with_rect(size.into()),
            HitboxConfig::Custom(rects) => {
                Hitbox::new().with_rects(rects.clone())
            }
        };
        entity = entity.with(hitbox);
    }

    // SOLID
    if let Some(is_solid) = tile_settings.is_solid {
        if is_solid {
            // TODO: Extract into config.
            entity = entity
                .with(Collidable::new(CollisionTag::from(vec![
                    CollisionLabel::Tile,
                    CollisionLabel::Solid,
                ])))
                .with(Solid::new(SolidTag::from(vec![
                    CollisionLabel::Tile,
                    CollisionLabel::Solid,
                ])));
        }
    }

    // JUMPPAD
    if tile_settings.jumppad.is_some()
        || tile_settings.jumppad_strength_x.is_some()
        || tile_settings.jumppad_strength_y.is_some()
    {
        // if let Some(mut jumppad) = tile_settings.jumppad.as_ref().cloned() {
        let mut jumppad = tile_settings
            .jumppad
            .as_ref()
            .cloned()
            .unwrap_or_else(Default::default);

        let strength: (Option<f32>, Option<f32>) = (
            tile_settings.jumppad_strength_x,
            tile_settings.jumppad_strength_y,
        );
        for axis in Axis::iter() {
            if let Some(axis_strength) = strength.by_axis(&axis) {
                *(&mut jumppad.strength).by_axis(&axis) = Some(axis_strength);
            }
        }

        entity = entity
            .with(Collidable::new(CollisionTag::from(CollisionLabel::Jumppad)))
            .with(jumppad);
    }

    // COLLISION TAG
    if let Some(collision_tag_wrapper) = &tile_settings.collision_tag {
        entity = entity.with(Collidable::new(CollisionTag::from(
            collision_tag_wrapper.clone(),
        )));
    }

    // SOLID TAG
    if let Some(solid_tag_wrapper) = &tile_settings.solid_tag {
        entity =
            entity.with(Solid::new(SolidTag::from(solid_tag_wrapper.clone())));
    }

    entity
}

pub(in super::super) static TILE_LOADABLE_PADDING: (Option<f32>, Option<f32>) =
    (Some(32.0), Some(32.0));

/// Adds base components to tile entity.
/// Components include:
///     - `Tile` // TODO
///     - `Transform`
///     - `Size`
///     - `ScaleOnce`
///     - `Transparent`
///     - `Loadable` (with padding)
///     - `Hidden`
///       Hidden, because the `EntityLoaderSystem` will add/remove
///       the Hidden component, depending on when it is loaded.
pub(super) fn base_tile_entity<'a>(
    world: &'a mut World,
    tile: &TileData,
    tile_size: SizeData,
) -> amethyst::Result<EntityBuilder<'a>> {
    const DEFAULT_Z: f32 = 0.0;

    let mut transform: Transform = tile.pos.into();
    transform.set_translation_z(tile.z_or(DEFAULT_Z));

    let size: Size = tile_size.into();
    let loadable = Loadable::default().with_padding(TILE_LOADABLE_PADDING);

    let entity = world
        .create_entity()
        // .with(Tile::default()) // TODO
        .with(transform)
        .with(size)
        .with(ScaleOnce::default())
        .with(Transparent)
        .with(loadable)
        .with(Hidden);

    Ok(entity)
}
