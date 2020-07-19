use super::system_prelude::*;

const PADDING: f32 = 8.0;

#[derive(Default)]
pub struct HandleAnimationsSystem;

impl<'a> System<'a> for HandleAnimationsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    );

    fn run(
        &mut self,
        (entities, velocities, mut animation_containers): Self::SystemData,
    ) {
        for (_, velocity, animations) in
            (&entities, &velocities, &mut animation_containers).join()
        {
            // TODO
            // handle other axis
            let _ = match velocity.x {
                x if x > PADDING => animations.play(AnimationKey::Walk),
                x if x < -PADDING => animations.play(AnimationKey::Walk),
                _ => animations.play(AnimationKey::Idle),
            };
        }
    }
}
