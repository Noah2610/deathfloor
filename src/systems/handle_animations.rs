use super::system_prelude::*;

const PADDING: f32 = 8.0;

#[derive(Default)]
pub struct HandleAnimationsSystem;

impl<'a> System<'a> for HandleAnimationsSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
    );

    fn run(
        &mut self,
        (entities, velocities, mut transforms, mut animation_containers): Self::SystemData,
    ) {
        for (_, transform, velocity, animations) in (
            &entities,
            &mut transforms,
            &velocities,
            &mut animation_containers,
        )
            .join()
        {
            // TODO
            // - handle other axis
            match velocity.x {
                x if x > PADDING => {
                    let scale = transform.scale_mut();
                    scale.x = scale.x.abs();
                    animations.play(AnimationKey::Walk)
                }
                x if x < -PADDING => {
                    let scale = transform.scale_mut();
                    scale.x = -scale.x.abs();
                    animations.play(AnimationKey::Walk)
                }
                _ => animations.play(AnimationKey::Idle),
            }
            .unwrap();
        }
    }
}
