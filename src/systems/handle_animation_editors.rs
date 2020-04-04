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
                match action {
                    AnimationAction::Play(key) => {
                        let errmsg = format!(
                            "[WARNING]\n    Couldn't play animation with key: \
                             {:?}",
                            &key
                        );
                        if let Err(e) = animations_container.play(key) {
                            eprintln!("{}\n    {}", errmsg, e);
                        }
                    }
                    AnimationAction::Push(key) => {
                        if animations_container
                            .current()
                            .map(|current_key| current_key != &key)
                            .unwrap_or(true)
                        {
                            let errmsg = format!(
                                "[WARNING]\n    Couldn't push animation with \
                                 key: {:?}",
                                &key
                            );
                            if let Err(e) = animations_container.push(key) {
                                eprintln!("{}\n    {}", errmsg, e);
                            }
                        }
                    }
                    AnimationAction::Pop => {
                        if let Err(e) = animations_container.pop() {
                            eprintln!("[WARNING]\n    Couldn't pop off animation\n    {}", e);
                        };
                    }
                }
            }
        }
    }
}
