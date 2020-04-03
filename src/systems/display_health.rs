use super::system_prelude::*;
use crate::helpers::resource;
use amethyst::core::math::Vector3;
// use amethyst::core::transform::Parent;
use std::collections::HashMap;
use std::path::PathBuf;

const SPRITE_NUMBER_RED: usize = 2;

#[derive(Clone, PartialEq)]
struct DisplayEntityData {
    parent_entity: Entity,
    health:        HitPoints,
    max_health:    HitPoints,
    pos:           [f32; 3],
    size:          (f32, f32),
}

#[derive(Debug)]
enum DisplayEntityUpdateAction {
    CreateNew,
    UpdateExisting(Entity),
    Ignore(Entity),
}

#[derive(Default)]
pub struct DisplayHealthSystem {
    display_entities: HashMap<Entity, (Entity, DisplayEntityData)>,
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
        // WriteStorage<'a, Parent>,
    );

    // TODO: REFACTOR
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
            // mut parent_store,
        ): Self::SystemData,
    ) {
        let colors_spritesheet_handle = spritesheets
            .get(&resource("spritesheets/colors.png"))
            .expect("colors.png spritesheet should be loaded at this point");

        let mut display_entities_to_create: Vec<DisplayEntityData> = Vec::new();

        for (entity, health, health_display, transform, size_opt) in (
            &entities,
            &health_store,
            &health_display_store,
            &transform_store,
            size_store.maybe(),
        )
            .join()
        {
            let pos: [f32; 3] = {
                let trans = transform.translation();
                let half_size: (f32, f32) =
                    size_opt.map(|s| (&s.half()).into()).unwrap_or((1.0, 1.0));

                let pos = match health_display.position {
                    HealthDisplayPosition::Top => [
                        trans.x,
                        trans.y
                            + half_size.1
                            + health_display.padding
                            + health_display.size.1 * 0.5,
                        trans.z + 0.01,
                    ],
                    HealthDisplayPosition::Bottom => [
                        trans.x,
                        trans.y
                            - half_size.1
                            - health_display.padding
                            - health_display.size.1 * 0.5,
                        trans.z + 0.01,
                    ],
                    // HealthDisplayPosition::Top => [
                    //     0.0,
                    //     0.0 + half_size.1
                    //         + health_display.padding
                    //         + health_display.size.1 * 0.5,
                    //     0.01,
                    // ],
                    // HealthDisplayPosition::Bottom => [
                    //     0.0,
                    //     0.0 - half_size.1
                    //         - health_display.padding
                    //         - health_display.size.1 * 0.5,
                    //     0.01,
                    // ],
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

            let display_entity_opt = {
                use DisplayEntityUpdateAction as Action;

                let display_entity_update_action: DisplayEntityUpdateAction =
                    self.display_entities
                        .get(&parent_entity)
                        .map(|(entity, previous_entity_data)| {
                            let entity = *entity;
                            if previous_entity_data != &display_entity_data {
                                if entities.is_alive(entity) {
                                    Action::UpdateExisting(entity)
                                } else {
                                    Action::CreateNew
                                }
                            } else {
                                Action::Ignore(entity)
                            }
                        })
                        .unwrap_or(Action::CreateNew);

                match display_entity_update_action {
                    Action::CreateNew => {
                        let display_entity = entities.create();
                        registered_display_entities.insert(
                            parent_entity,
                            (display_entity, display_entity_data.clone()),
                        );
                        Some(display_entity)
                    }
                    Action::UpdateExisting(display_entity) => {
                        registered_display_entities.insert(
                            parent_entity,
                            (display_entity, display_entity_data.clone()),
                        );
                        Some(display_entity)
                    }
                    Action::Ignore(display_entity) => {
                        registered_display_entities.insert(
                            parent_entity,
                            (display_entity, display_entity_data.clone()),
                        );
                        None
                    }
                }
            };

            if let Some(display_entity) = display_entity_opt {
                let size = Size::from(display_entity_data.size);
                let transform = {
                    let mut transform =
                        Transform::from(Vector3::from(display_entity_data.pos));
                    let scale = transform.scale_mut();
                    scale.x = size.w
                        * (display_entity_data.health as f32
                            / display_entity_data.max_health as f32);
                    scale.y = size.h;
                    transform
                };
                let sprite_render = SpriteRender {
                    sprite_number: SPRITE_NUMBER_RED,
                    sprite_sheet:  colors_spritesheet_handle.clone(),
                };

                // parent_store
                //     .insert(display_entity, Parent {
                //         entity: parent_entity,
                //     })
                //     .unwrap();
                transform_store.insert(display_entity, transform).unwrap();
                size_store.insert(display_entity, size).unwrap();
                sprite_render_store
                    .insert(display_entity, sprite_render)
                    .unwrap();
            }
        }

        self.display_entities = registered_display_entities;
    }
}
