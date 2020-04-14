use super::state_prelude::*;
use crate::input::prelude::{IngameActionBinding, IngameBindings};

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.insert(BulletCreator::default());
    }

    fn on_stop(&mut self, data: StateData<GameData<'a, 'b>>) {
        data.world.remove::<BulletCreator>();
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        if let Some(trans) = handle_input(&data.world) {
            return trans;
        }

        Trans::None
    }
}

fn handle_input<'a, 'b>(
    world: &World,
) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
    let input_manager = world.read_resource::<InputManager<IngameBindings>>();

    if input_manager.is_down(IngameActionBinding::TogglePause) {
        Some(Trans::Push(Box::new(Paused::default())))
    } else if input_manager.is_down(IngameActionBinding::ReloadLevel) {
        Some(Trans::Switch(Box::new(LoadIngame::default())))
    } else {
        None
    }
}
