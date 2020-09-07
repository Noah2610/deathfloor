use super::system_prelude::*;

#[derive(Default)]
pub struct HandleActionInsertComponents;

impl<'a> System<'a> for HandleActionInsertComponents {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, ActionTrigger<action::InsertComponents>>,
        EntityComponentsStorages<'a>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut action_trigger_store,
            mut components_storages,
        ): Self::SystemData,
    ) {
        for (entity, action_trigger) in
            (&entities, &mut action_trigger_store).join()
        {
            for action in action_trigger.drain_actions() {
                insert_components(entity, action.0, &mut components_storages)
                    .expect("Couldn't insert some components.");
            }
        }
    }
}
