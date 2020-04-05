use super::state_prelude::*;
use crate::helpers::resource;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        register_components(data.world);

        let mut sprite_sheet_handles =
            data.world.write_resource::<SpriteSheetHandles<PathBuf>>();
        sprite_sheet_handles
            .load(resource("spritesheets/player_bullet.png"), &data.world);
        sprite_sheet_handles
            .load(resource("spritesheets/colors.png"), &data.world);
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
