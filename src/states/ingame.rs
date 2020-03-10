use super::state_prelude::*;
use crate::input::prelude::{IngameActionBinding, IngameBindings};

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, _data: StateData<GameData<'a, 'b>>) {
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        #[cfg(feature = "debug")]
        {
            let input_manager =
                data.world.read_resource::<InputManager<IngameBindings>>();
            if input_manager.is_down(IngameActionBinding::ReloadLevel) {
                return Trans::Switch(Box::new(LoadIngame::default()));
            }
        }

        Trans::None
    }
}
