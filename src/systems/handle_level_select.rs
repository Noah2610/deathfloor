use super::system_prelude::*;
use crate::states::level_select::SelectLevel;
use amethyst::ui::{UiText, UiTransform};

const UI_BTN_ID: &str = "btn_select_level_btn_txt";
const UI_IDX_ID: &str = "level_select_index";

#[derive(Default)]
pub struct HandleLevelSelectSystem {
    selected_idx: usize,
}

impl<'a> System<'a> for HandleLevelSelectSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        ReadExpect<'a, InputManager<MenuBindings>>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
        Write<'a, SelectLevel>,
    );

    fn run(
        &mut self,
        (
            settings,
            input_manager,
            ui_transform_store,
            mut ui_text_store,
            mut select_level,
        ): Self::SystemData,
    ) {
        let level_settings = &settings.level;
        let levels_len = level_settings.levels.len();

        let mut selected_level_name =
            level_settings.levels[self.selected_idx].clone();

        if input_manager.is_down(MenuActionBinding::Select) {
            select_level.0 = Some(selected_level_name.clone());
        } else {
            let next_idx_opt = if input_manager.is_down(MenuActionBinding::Next)
            {
                Some((self.selected_idx + 1).rem_euclid(levels_len))
            } else if input_manager.is_down(MenuActionBinding::Prev) {
                Some(
                    ((self.selected_idx as i32 - 1)
                        .rem_euclid(levels_len as i32))
                        as usize,
                )
            } else {
                None
            };

            if let Some(next_idx) = next_idx_opt {
                self.selected_idx = next_idx;
                selected_level_name =
                    level_settings.levels[self.selected_idx].clone();
            }
        }

        for (ui_transform, ui_text) in
            (&ui_transform_store, &mut ui_text_store).join()
        {
            // LEVEL NAME
            if &ui_transform.id == UI_BTN_ID {
                ui_text.text =
                    selected_level_name.replace(".json", "").replace("_", " ");
            } else
            // LEVEL INDEX
            if &ui_transform.id == UI_IDX_ID {
                ui_text.text =
                    format!("{} / {}", self.selected_idx + 1, levels_len);
            }
        }
    }
}
