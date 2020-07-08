use super::state_prelude::*;
use std::path::PathBuf;

pub struct LoadIngame {
    level_path: PathBuf,
}

impl LoadIngame {
    pub fn new<P>(level_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            level_path: level_path.into(),
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LoadIngame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();
        crate::map_loader::load_map(data.world, self.level_path.clone())
            .unwrap();
    }

    fn update(
        &mut self,
        _data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        Trans::Switch(Box::new(Ingame::new(self.level_path.clone())))
    }
}
