use super::state_prelude::*;
use crate::helpers::resource;
use crate::input::menu_bindings::*;
use amethyst::ui::{UiEvent, UiEventType};
use deathframe::core::menu::prelude::*;

const BGM: SongType = SongType::Cntrlgun;

#[derive(Default)]
pub struct MainMenu {
    ui_data: UiData,
}

impl MainMenu {
    fn start<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.create_ui(data, resource("ui/main_menu.ron").to_str().unwrap());
        data.world.write_resource::<Songs<SongType>>().play(&BGM);
    }

    fn stop<'a, 'b>(&mut self, data: &mut StateData<GameData<'a, 'b>>) {
        self.delete_ui(data);
        data.world.write_resource::<Songs<SongType>>().stop(&BGM);
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for MainMenu {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_str() {
                "btn_start" => {
                    Some(Trans::Push(Box::new(LevelSelect::default())))
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

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for MainMenu {
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
        data.data
            .update(data.world, DispatcherId::MainMenu)
            .unwrap();

        let input = data.world.read_resource::<InputManager<MenuBindings>>();
        if input.is_down(MenuActionBinding::Select) {
            return Trans::Push(Box::new(LevelSelect::default()));
        } else if input.is_down(MenuActionBinding::Quit) {
            return Trans::Quit;
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
