use super::system_prelude::*;

#[derive(Default)]
pub struct HandleLockedToPathSystem;

impl<'a> System<'a> for HandleLockedToPathSystem {
    type SystemData =
        (ReadStorage<'a, LockedToPath>, WriteStorage<'a, Transform>);

    fn run(
        &mut self,
        (locked_to_path_store, mut transform_store): Self::SystemData,
    ) {
        for (locked_to_path, transform) in
            (&locked_to_path_store, &mut transform_store).join()
        {
            let pos = {
                let trans = transform.translation();
                (trans.x, trans.y)
            };

            // Find closest point

            let mut closest: Option<((f32, f32), f32)> = None;
            for point in &locked_to_path.path {
                let dist = ((point.x - pos.0).powf(2.0)
                    + (point.y - pos.1).powf(2.0))
                .sqrt();

                let is_closest =
                    closest.map(|closest| dist < closest.1).unwrap_or(true);
                if is_closest {
                    closest = Some(((point.x, point.y), dist));
                }
            }

            if let Some((closest_pos, _)) = closest {
                transform.set_translation_x(closest_pos.0);
                transform.set_translation_y(closest_pos.1);
            }
        }
    }
}
