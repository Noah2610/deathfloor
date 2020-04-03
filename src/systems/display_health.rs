use super::system_prelude::*;
use crate::helpers::resource;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Parent;
use std::collections::HashMap;
use std::path::PathBuf;

const SPRITE_NUMBER_BLACK: usize = 0;
const SPRITE_NUMBER_WHITE: usize = 1;
const SPRITE_NUMBER_RED: usize = 2;

#[derive(Clone, PartialEq)]
struct DisplayEntityData {
    parent_entity:  Entity,
    health:         HitPoints,
    max_health:     HitPoints,
    pos:            [f32; 3],
    size:           (f32, f32),
    border_padding: f32,
}

#[derive(Clone, Copy)]
struct DisplayEntities {
    wrapper:    Entity,
    health_bar: Entity,
    background: Entity,
}

enum DisplayEntityUpdateAction {
    CreateNew,
    UpdateExisting(DisplayEntities),
    Ignore(DisplayEntities),
}

#[derive(Default)]
pub struct DisplayHealthSystem {
    display_entities: HashMap<Entity, (DisplayEntities, DisplayEntityData)>,
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
            mut parent_store,
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
                parent_entity:  entity,
                health:         health.health,
                max_health:     health.max_health,
                pos:            pos,
                size:           health_display.size,
                border_padding: health_display.border_padding,
            };

            display_entities_to_create.push(display_entity_data);
        }

        let mut registered_display_entities = HashMap::default();

        for display_entity_data in display_entities_to_create {
            let parent_entity = display_entity_data.parent_entity;

            let display_entities_opt = {
                use DisplayEntityUpdateAction as Action;

                let display_entity_update_action: DisplayEntityUpdateAction =
                    self.display_entities
                        .get(&parent_entity)
                        .map(|(display_entities, previous_entity_data)| {
                            let display_entities = *display_entities;
                            if previous_entity_data != &display_entity_data {
                                if entities.is_alive(display_entities.wrapper) {
                                    Action::UpdateExisting(display_entities)
                                } else {
                                    Action::CreateNew
                                }
                            } else {
                                Action::Ignore(display_entities)
                            }
                        })
                        .unwrap_or(Action::CreateNew);

                match display_entity_update_action {
                    Action::CreateNew => {
                        let display_entities = DisplayEntities {
                            wrapper:    entities.create(),
                            health_bar: entities.create(),
                            background: entities.create(),
                        };
                        registered_display_entities.insert(
                            parent_entity,
                            (display_entities, display_entity_data.clone()),
                        );
                        Some(display_entities)
                    }
                    Action::UpdateExisting(display_entities) => {
                        registered_display_entities.insert(
                            parent_entity,
                            (display_entities, display_entity_data.clone()),
                        );
                        Some(display_entities)
                    }
                    Action::Ignore(display_entities) => {
                        registered_display_entities.insert(
                            parent_entity,
                            (display_entities, display_entity_data.clone()),
                        );
                        None
                    }
                }
            };

            if let Some(display_entities) = display_entities_opt {
                let size = Size::from(display_entity_data.size);
                let [parent_transform, bar_transform, bg_transform] = {
                    let parent_transform =
                        Transform::from(Vector3::from(display_entity_data.pos));
                    let [mut bar_transform, mut bg_transform] =
                        { [Transform::default(), Transform::default()] };
                    bar_transform.translation_mut().z += 0.001;
                    let bar_scale = bar_transform.scale_mut();
                    bar_scale.x = size.w
                        * (display_entity_data.health as f32
                            / display_entity_data.max_health as f32)
                        - display_entity_data.border_padding;
                    bar_scale.y = size.h - display_entity_data.border_padding;
                    let bg_scale = bg_transform.scale_mut();
                    bg_scale.x = size.w;
                    bg_scale.y = size.h;
                    [parent_transform, bar_transform, bg_transform]
                };
                let bar_sprite_render = SpriteRender {
                    sprite_number: SPRITE_NUMBER_RED,
                    sprite_sheet:  colors_spritesheet_handle.clone(),
                };
                let bg_sprite_render = SpriteRender {
                    sprite_number: SPRITE_NUMBER_WHITE,
                    sprite_sheet:  colors_spritesheet_handle.clone(),
                };

                let display_parent = Parent {
                    entity: display_entities.wrapper,
                };

                // PARENT ENTITY
                transform_store
                    .insert(display_entities.wrapper, parent_transform)
                    .unwrap();
                size_store.insert(display_entities.wrapper, size).unwrap();
                // BACKGROUND ENTITY
                parent_store
                    .insert(display_entities.background, display_parent.clone())
                    .unwrap();
                transform_store
                    .insert(display_entities.background, bg_transform)
                    .unwrap();
                sprite_render_store
                    .insert(display_entities.background, bg_sprite_render)
                    .unwrap();
                // HEALTH BAR ENTITY
                parent_store
                    .insert(display_entities.health_bar, display_parent)
                    .unwrap();
                transform_store
                    .insert(display_entities.health_bar, bar_transform)
                    .unwrap();
                sprite_render_store
                    .insert(display_entities.health_bar, bar_sprite_render)
                    .unwrap();
            }
        }

        // Remove old entities
        for (prev_parent_entity, (prev_display_entities, _data)) in
            self.display_entities.iter()
        {
            if !registered_display_entities.contains_key(prev_parent_entity) {
                entities.delete(prev_display_entities.background).unwrap();
                entities.delete(prev_display_entities.health_bar).unwrap();
                entities.delete(prev_display_entities.wrapper).unwrap();
            }
        }

        self.display_entities = registered_display_entities;
    }
}
