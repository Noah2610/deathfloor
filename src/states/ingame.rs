use super::state_prelude::*;
use crate::input::prelude::{IngameActionBinding, IngameBindings};
use std::path::PathBuf;

const BGM: SongType = SongType::Floor1;

pub struct Ingame {
    level_path: PathBuf,
}

impl Ingame {
    pub fn new<P>(level_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            level_path: level_path.into(),
        }
    }

    fn handle_input<'a, 'b>(
        &self,
        world: &World,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        let input_manager =
            world.read_resource::<InputManager<IngameBindings>>();

        if input_manager.is_down(IngameActionBinding::TogglePause) {
            Some(Trans::Push(Box::new(Paused::default())))
        } else if input_manager.is_down(IngameActionBinding::Quit) {
            Some(Trans::Pop)
        } else if input_manager.is_down(IngameActionBinding::ReloadLevel) {
            Some(Trans::Switch(Box::new(LoadIngame::new(
                self.level_path.clone(),
            ))))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.insert(BulletCreator::default());
        data.world.write_resource::<Songs<SongType>>().play(&BGM);
    }

    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.remove::<BulletCreator>();
        data.world.write_resource::<Songs<SongType>>().stop(&BGM);
    }

    fn on_resume(&mut self, data: StateData<GameData<'a, 'b>>) {
        let _ = data.world.write_resource::<Songs<SongType>>().resume(&BGM);
    }

    fn on_pause(&mut self, data: StateData<GameData<'a, 'b>>) {
        let _ = data.world.write_resource::<Songs<SongType>>().pause(&BGM);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        if let Some(trans) = self.handle_input(&data.world) {
            return trans;
        }

        Trans::None
    }
}
