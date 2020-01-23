mod shape_wrapper;

pub mod prelude {
    pub use super::shape_wrapper::ShapeWrapper;
    pub use super::ColliderWrapper;
}

use deathframe::specs_physics;
use prelude::*;
use specs_physics::ncollide::shape::ShapeHandle;
use specs_physics::nphysics::material::{BasicMaterial, MaterialHandle};
use specs_physics::nphysics::object::ColliderDesc;

const DEFAULT_SOLID_MARGIN: f32 = 0.01;

#[derive(Clone, Deserialize)]
pub struct ColliderWrapper {
    /// See `specs_physics::ncollide::shape`.
    pub shape: ShapeWrapper,

    /// The material's friction.
    pub friction: f32,

    /// The material's restitution.
    pub restitution: f32,

    /// The solid margin for the collider.
    /// Should be low.
    /// Consider subtracting the shape's size by the solid_margin.
    /// Defaults to 0.01
    #[serde(default)]
    pub solid_margin: Option<f32>,
}

impl ColliderWrapper {
    fn shape(&self) -> ShapeHandle<f32> {
        (&self.shape).into()
    }

    fn material(&self) -> MaterialHandle<f32> {
        MaterialHandle::new(BasicMaterial::new(self.restitution, self.friction))
    }
}

impl Into<ColliderDesc<f32>> for &ColliderWrapper {
    fn into(self) -> ColliderDesc<f32> {
        ColliderDesc::new(self.shape())
            .margin(self.solid_margin.unwrap_or(DEFAULT_SOLID_MARGIN))
            .material(self.material())
    }
}
