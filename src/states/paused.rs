use super::state_prelude::*;
use crate::input::prelude::{PausedActionBinding, PausedBindings};

#[derive(Default)]
pub struct Paused;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Paused {
    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Paused).unwrap();

        let input_manager =
            data.world.read_resource::<InputManager<PausedBindings>>();
        if input_manager.is_down(PausedActionBinding::TogglePause) {
            return Trans::Pop;
        }

        Trans::None
    }
}
