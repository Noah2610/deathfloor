use super::system_prelude::*;

#[derive(Default)]
pub struct HandleAnimationEditorsSystem;

impl<'a> System<'a> for HandleAnimationEditorsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationEditor>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut animation_editor_store,
            mut animations_container_store,
        ): Self::SystemData,
    ) {
        for (_entity, animation_editor, animations_container) in (
            &entities,
            &mut animation_editor_store,
            &mut animations_container_store,
        )
            .join()
        {
            for action in animation_editor.drain_actions() {
                let _ = match action {
                    AnimationAction::Play(key) => {
                        animations_container.play(key)
                    }
                    AnimationAction::Push(key) => {
                        animations_container.push(key)
                    }
                    AnimationAction::Pop => {
                        animations_container.pop().map(|_| ())
                    }
                }
                .map_err(|e| eprintln!("[WARNING]\n    {}", e));
            }
        }
    }
}
