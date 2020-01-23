use deathframe::geo::Vector;
use deathframe::specs_physics;
use specs_physics::ncollide::shape::{self, ShapeHandle};

#[derive(Clone, Deserialize)]
pub enum ShapeWrapper {
    Ball { radius: f32 },
    Cuboid { half_width: f32, half_height: f32 },
}

impl Into<ShapeHandle<f32>> for &ShapeWrapper {
    fn into(self) -> ShapeHandle<f32> {
        match self {
            ShapeWrapper::Ball { radius } => {
                ShapeHandle::new(shape::Ball::new(*radius))
            }
            ShapeWrapper::Cuboid {
                half_width,
                half_height,
            } => ShapeHandle::new(shape::Cuboid::new(Vector::new(
                *half_width,
                *half_height,
            ))),
        }
    }
}
