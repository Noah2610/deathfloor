use super::system_prelude::*;

const PADDING: f32 = 0.5;

#[derive(Default)]
pub struct HandleScalesSystem;

impl<'a> System<'a> for HandleScalesSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Facing>,
    );

    fn run(
        &mut self,
        (velocity_store, mut transform_store, facing_store): Self::SystemData,
    ) {
        for (transform, velocity, facing_opt) in
            (&mut transform_store, &velocity_store, facing_store.maybe()).join()
        {
            match facing_opt {
                Some(Facing::Left) => {
                    let scale = transform.scale_mut();
                    scale.x = -scale.x.abs();
                }
                Some(Facing::Right) => {
                    let scale = transform.scale_mut();
                    scale.x = scale.x.abs();
                }
                None => match velocity.x {
                    x if x > PADDING => {
                        let scale = transform.scale_mut();
                        scale.x = scale.x.abs();
                    }
                    x if x < -PADDING => {
                        let scale = transform.scale_mut();
                        scale.x = -scale.x.abs();
                    }
                    _ => (),
                },
            }
        }
    }
}
