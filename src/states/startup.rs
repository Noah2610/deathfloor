use super::state_prelude::*;
use crate::helpers::resource;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        register_components(data.world);
        load_spritesheets(&mut data.world);
        load_audio(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(LoadIngame::default()))
    }
}

fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Camera>();
    world.register::<Shooter>();
    world.register::<Jumppad>();
    world.register::<JumppadAffected>();
    world.register::<Enemy>();
    world.register::<Lifecycle>();
}

fn load_spritesheets(world: &mut World) {
    let mut sprite_sheet_handles =
        world.write_resource::<SpriteSheetHandles<PathBuf>>();
    sprite_sheet_handles
        .load(resource("spritesheets/player_bullet.png"), world);
    sprite_sheet_handles.load(resource("spritesheets/colors.png"), world);
}

fn load_audio(world: &mut World) {
    let mut sounds = Sounds::default();
    sounds.load_sounds(&world.read_resource(), &world.read_resource());
    world.insert(sounds);
}
