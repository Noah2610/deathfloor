use super::system_prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct PlayDeathAnimationBeforeDeletionSystem {
    dying_entities: HashSet<Entity>,
}

impl<'a> System<'a> for PlayDeathAnimationBeforeDeletionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        WriteStorage<'a, Lifecycle>,
    );

    fn run(
        &mut self,
        (entities, mut animations_store, mut lifecycle_store): Self::SystemData,
    ) {
        for (entity, animations, lifecycle) in
            (&entities, &mut animations_store, &mut lifecycle_store).join()
        {
            if animations.has_animation(&AnimationKey::Death) {
                let is_dying = self.dying_entities.contains(&entity);

                if is_dying {
                    if let Some(&AnimationKey::Death) = animations.current() {
                        lifecycle.prolong(1);
                    }
                } else {
                    if &lifecycle.state == &LifecycleState::Death {
                        self.dying_entities.insert(entity);
                        animations.push(AnimationKey::Death).unwrap();
                        lifecycle.prolong(1);
                    }
                }
            }
        }
    }
}
