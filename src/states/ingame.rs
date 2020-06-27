use super::state_prelude::*;
use crate::input::prelude::{IngameActionBinding, IngameBindings};

const BGM: SongType = SongType::Floor1;

#[derive(Default)]
pub struct Ingame;

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
