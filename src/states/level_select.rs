use super::state_prelude::*;
use crate::helpers::resource;
use amethyst::ui::{UiEvent, UiEventType};
use deathframe::core::menu::prelude::*;

#[derive(Default)]
pub struct SelectLevel(pub Option<String>);

#[derive(Default)]
pub struct LevelSelect {
    ui_data: UiData,
}

impl LevelSelect {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/level_select.ron").to_str().unwrap());
        data.world.insert(SelectLevel::default());
    }

    fn stop<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.delete_ui(data);
        data.world.remove::<LevelSelect>();
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LevelSelect {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.stop(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_only(data.world, DispatcherId::Ui).unwrap();
        data.data
            .update_only(data.world, DispatcherId::LevelSelect)
            .unwrap();
        data.data.update_core(data.world);

        if let Some(selected_level_name) =
            data.world.read_resource::<SelectLevel>().0.as_ref()
        {
            let level_path = resource("levels").join(selected_level_name);
            return Trans::Push(Box::new(LoadIngame::new(level_path)));
        }

        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            trans
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for LevelSelect {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_str() {
                "btn_start" => {
                    Some(Trans::Push(Box::new(default_load_ingame())))
                }
                "btn_quit" => Some(Trans::Quit),
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}

// TODO
fn default_load_ingame() -> LoadIngame {
    use std::env::args;

    LoadIngame::new(resource(format!(
        "levels/{}",
        args().skip(1).next().unwrap_or("level.json".to_string())
    )))
}
