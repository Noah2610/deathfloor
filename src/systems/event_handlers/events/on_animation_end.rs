use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleEventOnAnimationEnd {
    last_finished_anims: HashMap<Entity, AnimationKey>,
}

impl<'a> System<'a> for HandleEventOnAnimationEnd {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, EventsRegister>,
        WriteStorage<'a, ActionTypeTrigger>,
        ReadStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, Loadable>,
        ReadStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            events_register_store,
            mut action_type_trigger_store,
            animations_container_store,
            loadable_store,
            loaded_store,
        ): Self::SystemData,
    ) {
        for (
            entity,
            events_register,
            action_type_trigger,
            animations,
            loadable_opt,
            loaded_opt,
        ) in (
            &entities,
            &events_register_store,
            &mut action_type_trigger_store,
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
                        if let Some(last_anim) =
                            animations.last_finished_animation()
                        {
                            if last_anim == target_anim
                                && self
                                    .last_finished_anims
                                    .get(&entity)
                                    .map(|saved_last_anim| {
                                        saved_last_anim != last_anim
                                    })
                                    .unwrap_or(true)
                            {
                                action_type_trigger.add_action(action.clone());
                                self.last_finished_anims
                                    .insert(entity, last_anim.clone());
                            }
                        } else {
                            let _ = self.last_finished_anims.remove(&entity);
                        }
                    }
                }
            }
        }
    }
}
