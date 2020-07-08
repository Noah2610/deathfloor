use super::system_prelude::*;
use amethyst::ui::{UiText, UiTransform};

const UI_BTN_SELECT_ID: &str = "btn_select_level_btn_txt";

#[derive(Default)]
pub struct HandleLevelSelectSystem {
    selected_idx: usize,
}

impl<'a> System<'a> for HandleLevelSelectSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Read<'a, InputManager<MenuBindings>>,
        ReadStorage<'a, UiTransform>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (
            settings,
            input_manager,
            ui_transform_store,
            mut ui_text_store,
        ): Self::SystemData,
    ) {
        let level_settings = &settings.level;
        let levels_len = level_settings.levels.len();

        let next_idx_opt = if input_manager.is_down(MenuActionBinding::Next) {
            Some((self.selected_idx + 1).rem_euclid(levels_len))
        } else if input_manager.is_down(MenuActionBinding::Prev) {
            Some(
                ((self.selected_idx as i32 - 1).rem_euclid(levels_len as i32))
                    as usize,
            )
        } else {
            None
        };

        if let Some(next_idx) = next_idx_opt {
            self.selected_idx = next_idx;

            for (ui_transform, ui_text) in
                (&ui_transform_store, &mut ui_text_store).join()
            {
                if &ui_transform.id == UI_BTN_SELECT_ID {
                    ui_text.text = level_settings.levels[self.selected_idx]
                        .clone()
                        .replace(".json", "")
                        .replace("_", " ");
                }
            }
        }
    }
}
