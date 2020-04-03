use super::system_prelude::*;
use crate::helpers::resource;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Parent;
use std::collections::HashMap;
use std::path::PathBuf;

const SPRITE_NUMBER_RED: usize = 2;

#[derive(Default)]
pub struct DisplayHealthSystem {
    display_entities: HashMap<Entity, Entity>,
}

impl<'a> System<'a> for DisplayHealthSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, SpriteSheetHandles<PathBuf>>,
        ReadStorage<'a, Health>,
        ReadStorage<'a, HealthDisplay>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Parent>,
    );

    fn run(
        &mut self,
        (
            entities,
            spritesheets,
            health_store,
            health_display_store,
            mut transform_store,
            mut size_store,
            mut sprite_render_store,
            mut parent_store,
        ): Self::SystemData,
    ) {
        struct DisplayEntityData {
            parent_entity: Entity,
            health:        HitPoints,
            max_health:    HitPoints,
            pos:           [f32; 3],
            size:          (f32, f32),
        }

        let colors_spritesheet_handle = spritesheets
            .get(&resource("spritesheets/colors.png"))
            .expect("colors.png spritesheet should be loaded at this point");

        let mut display_entities_to_create: Vec<DisplayEntityData> = Vec::new();

        for (entity, health, health_display, size_opt) in (
            &entities,
            &health_store,
            &health_display_store,
            size_store.maybe(),
        )
            .join()
        {
            let pos: [f32; 3] = {
                let half_size: (f32, f32) =
                    size_opt.map(|s| (&s.half()).into()).unwrap_or((1.0, 1.0));

                let pos = match health_display.position {
                    HealthDisplayPosition::Top => [
                        0.0,
                        0.0 + half_size.1
                            + health_display.padding
                            + health_display.size.1 * 0.5,
                        0.01,
                    ],
                    HealthDisplayPosition::Bottom => [
                        0.0,
                        0.0 - half_size.1
                            - health_display.padding
                            - health_display.size.1 * 0.5,
                        0.01,
                    ],
                };

                pos
            };

            let display_entity_data = DisplayEntityData {
                parent_entity: entity,
                health:        health.health,
                max_health:    health.max_health,
                pos:           pos,
                size:          health_display.size,
            };

            display_entities_to_create.push(display_entity_data);
        }

        let mut registered_display_entities = HashMap::default();

        for display_entity_data in display_entities_to_create {
            let parent_entity = display_entity_data.parent_entity;

            let display_entity = self
                .display_entities
                .get(&parent_entity)
                .cloned()
                .and_then(|entity| {
                    if entities.is_alive(entity) {
                        Some(entity)
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| entities.create());

            let transform =
                Transform::from(Vector3::from(display_entity_data.pos));
            let size = Size::from(display_entity_data.size);
            let sprite_render = SpriteRender {
                sprite_number: SPRITE_NUMBER_RED,
                sprite_sheet:  colors_spritesheet_handle.clone(),
            };

            parent_store
                .insert(display_entity, Parent {
                    entity: parent_entity,
                })
                .unwrap();
            transform_store.insert(display_entity, transform).unwrap();
            size_store.insert(display_entity, size).unwrap();
            sprite_render_store
                .insert(display_entity, sprite_render)
                .unwrap();

            registered_display_entities.insert(parent_entity, display_entity);
        }

        self.display_entities = registered_display_entities;
    }
}