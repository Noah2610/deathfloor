use super::state_prelude::*;
use crate::helpers::resource;

#[derive(Default)]
pub struct LoadIngame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LoadIngame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();
        crate::map_loader::load_map(data.world, resource("levels/level.json"))
            .unwrap();
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(Ingame::default()))
    }
}
