use super::system_prelude::*;
use crate::map_loader::map_data::Pos;

#[derive(Default)]
pub struct HandleActionSpawnAction;

impl<'a> System<'a> for HandleActionSpawnAction {
    type SystemData = (
        WriteStorage<'a, ActionTrigger<action::SpawnAction>>,
        Write<'a, ObjectSpawner>,
        ReadStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (
            mut action_trigger_store,
            mut object_spawner,
            transform_store,
        ): Self::SystemData,
    ) {
        for (action_trigger, transform) in
            (&mut action_trigger_store, &transform_store).join()
        {
            let source_pos = transform.translation();

            for action in action_trigger.drain_actions() {
                match action {
                    action::SpawnAction::SpawnRelative(
                        mut object_spawn_data,
                    ) => {
                        let target_pos = Pos {
                            x: source_pos.x + object_spawn_data.object.pos.x,
                            y: source_pos.y + object_spawn_data.object.pos.y,
                        };
                        object_spawn_data.object.pos = target_pos;
                        object_spawner.add(object_spawn_data);
                    }
                }
            }
        }
    }
}
