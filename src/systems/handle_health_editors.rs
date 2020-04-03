use super::system_prelude::*;

#[derive(Default)]
pub struct HandleHealthEditorsSystem;

impl<'a> System<'a> for HandleHealthEditorsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Health>,
        WriteStorage<'a, HealthEditor>,
    );

    fn run(
        &mut self,
        (entities, mut health_store, mut health_editor_store): Self::SystemData,
    ) {
        for (entity, health, health_editor) in
            (&entities, &mut health_store, &mut health_editor_store).join()
        {
            for action in health_editor.drain_actions() {
                match action {
                    HealthAction::Gain(hp) => {
                        health.gain(hp);
                    }
                    HealthAction::Lose(hp) => {
                        health.lose(hp);
                    }
                }
            }
        }
    }
}
