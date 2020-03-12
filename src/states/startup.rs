use super::state_prelude::*;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        register_components(data.world);
        insert_resources(data.world).expect("failed to load resources");
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
    world.register::<Jumppad>();
    world.register::<JumppadAffected>();
}
