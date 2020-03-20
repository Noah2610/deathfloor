use super::system_prelude::*;

const PADDING: f32 = 0.5;

#[derive(Default)]
pub struct HandleScalesSystem;

impl<'a> System<'a> for HandleScalesSystem {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Transform>);

    fn run(&mut self, (velocities, mut transforms): Self::SystemData) {
        for (transform, velocity) in (&mut transforms, &velocities).join() {
            match velocity.x {
                x if x > PADDING => {
                    let scale = transform.scale_mut();
                    scale.x = scale.x.abs();
                }
                x if x < -PADDING => {
                    let scale = transform.scale_mut();
                    scale.x = -scale.x.abs();
                }
                _ => (),
            }
        }
    }
}
