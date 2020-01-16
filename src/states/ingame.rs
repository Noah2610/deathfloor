use super::state_prelude::*;
use crate::helpers::resource;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.delete_all();

        crate::map_loader::load_map(data.world, resource("levels/level.json"))
            .unwrap();

        // let player_entity = create_player(data.world);
        // // create_camera(data.world, Some(player_entity));
        // create_camera(data.world, None);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();
        Trans::None
    }
}
