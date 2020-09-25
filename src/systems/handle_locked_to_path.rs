use super::system_prelude::*;
use crate::map_loader::map_data::prelude::PosData;
use std::cmp;

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
            // We refer to the entity's position as the _client_.
            let client = {
                let trans = transform.translation();
                Pos {
                    x: trans.x as Float,
                    y: trans.y as Float,
                }
            };

            // 1) Create triangle.
            //    Find the two points on the path, where the client pos is in the middle.
            //    Check path points in pairs of two:
            //    Check if point A is to the left of client and point B is to the right of client.
            //
            // 2) Calculate triangle values.
            //    - area of triangle
            //      AREA = (a * b * sin(C)) / 2
            //        - C:   radius of client point
            //        - a,b: edges touching client point
            //    - HEIGHT-c = (2 * AREA) / c
            //        - c: edge opposite to client
            //
            // 3) Calculate target pos
            //    TARGET = CLIENT-POS + HEIGHT-c

            // 1)
            let points_a_and_b = (0 .. locked_to_path.path.len())
                .into_iter()
                .find_map(|path_idx_a| {
                    let path_idx_b = path_idx_a + 1;
                    if let (Some(point_a), Some(point_b)) = (
                        locked_to_path.path.get(path_idx_a),
                        locked_to_path.path.get(path_idx_b),
                    ) {
                        let (point_a, point_b) =
                            (Pos::from(point_a), Pos::from(point_b));
                        if point_a < client && client < point_b {
                            let y_sign = if client.x - point_a.x
                                < point_b.x - client.x
                            {
                                // point_a is closer on the x
                                (point_a.y - client.y).signum()
                            } else {
                                // point_b is closer on the x
                                (point_b.y - client.y).signum()
                            };
                            Some((point_a, point_b, y_sign))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });

            // 2)
            let target_pos = if let Some((point_a, point_b, y_sign)) =
                points_a_and_b
            {
                let point_c = client;

                let edge_c = ((point_a.x - point_b.x).powf(2.0)
                    + (point_a.y - point_b.y).powf(2.0))
                .sqrt();
                if edge_c.is_nan() {
                    eprintln!("2) edge_c");
                }
                let edge_a = ((point_b.x - point_c.x).powf(2.0)
                    + (point_b.y - point_c.y).powf(2.0))
                .sqrt();
                if edge_a.is_nan() {
                    eprintln!("2) edge_a");
                }
                let edge_b = ((point_c.x - point_a.x).powf(2.0)
                    + (point_c.y - point_a.y).powf(2.0))
                .sqrt();
                if edge_b.is_nan() {
                    eprintln!("2) edge_b");
                }

                let corner_a = ((edge_b.powf(2.0) + edge_c.powf(2.0)
                    - edge_a.powf(2.0))
                    / (2.0 * edge_b * edge_c))
                    .max(-1.0)
                    .min(1.0)
                    .acos();
                if corner_a.is_nan() {
                    eprintln!("2) corner_a");
                }
                let corner_b = ((edge_a.powf(2.0) + edge_c.powf(2.0)
                    - edge_b.powf(2.0))
                    / (2.0 * edge_a * edge_c))
                    .max(-1.0)
                    .min(1.0)
                    .acos();
                if corner_b.is_nan() {
                    eprintln!("2) corner_b");
                }
                let corner_c = ((edge_a.powf(2.0) + edge_b.powf(2.0)
                    - edge_c.powf(2.0))
                    / (2.0 * edge_a * edge_b))
                    .max(-1.0)
                    .min(1.0)
                    .acos();
                if corner_c.is_nan() {
                    eprintln!("2) corner_c");
                }

                let area = (edge_a * edge_b * corner_c.sin()) / 2.0;

                let height_c = (2.0 * area) / edge_c;

                // 3)
                Some({
                    let new_edge_a = height_c;
                    let new_edge_b = edge_b;
                    let new_corner_a = corner_a;
                    let new_corner_b = ((new_edge_b * new_corner_a.sin())
                        / new_edge_a)
                        .max(-1.0)
                        .min(1.0)
                        .asin();
                    if new_corner_b.is_nan() {
                        eprintln!("3) new_corner_b");
                    }
                    let new_corner_c = (180.0 as Float).to_radians()
                        - new_corner_a
                        - new_corner_b;

                    let dist_cb = new_edge_a;
                    let x = point_c.x + (dist_cb * new_corner_b.cos());
                    let y = point_c.y + (dist_cb * new_corner_b.sin() * y_sign);
                    Pos { x, y }
                })
            } else {
                None
            };

            // Apply calculated target pos.
            if let Some(target_pos) = target_pos {
                transform.set_translation_x(target_pos.x as f32);
                transform.set_translation_y(target_pos.y as f32);
            } else {
                eprintln!("No target_pos");
            }
        }
    }
}

type Float = f32;

#[derive(PartialEq)]
struct Pos {
    x: Float,
    y: Float,
}

impl<'a> From<&'a PosData> for Pos {
    fn from(data: &'a PosData) -> Self {
        Self {
            x: data.x as Float,
            y: data.y as Float,
        }
    }
}

impl cmp::PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.x.partial_cmp(&other.x)
    }
}
