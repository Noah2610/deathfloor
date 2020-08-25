use super::system_prelude::*;

#[derive(Default)]
pub struct HandleEventOnAnimationEnd;

impl<'a> System<'a> for HandleEventOnAnimationEnd {
    type SystemData = (
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, Animation>,
        ReadStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            events_register_store,
            mut action_type_trigger_store,
            animation_store,
            animations_container_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (
            events_register,
            action_type_trigger,
            animation,
            animations,
            loadable_opt,
            loaded_opt,
        ) in (
            &events_register_store,
            &mut action_type_trigger_store,
            &animation_store,
            &animations_container_store,
            loadable_store.maybe(),
            loaded_store.maybe(),
        )
            .join()
        {
            if let (Some(_), Some(_)) | (None, None) =
                (loadable_opt, loaded_opt)
            {
                for (event, action) in events_register.events().iter() {
                    if let EventType::OnAnimationEnd(target_anim) = event {
                        if let Some(current) = animations.current() {
                            if current == target_anim {
                                if animation.has_played_and_is_finished() {
                                    action_type_trigger
                                        .add_action(action.clone());
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
