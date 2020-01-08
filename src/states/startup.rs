use super::state_prelude::*;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        register_components(data.world);
        init_resources(data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(Ingame::default()))
    }
}

fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Camera>();
}

fn init_resources(world: &mut World) {
    world.insert(SpriteSheetHandles::default());
}
