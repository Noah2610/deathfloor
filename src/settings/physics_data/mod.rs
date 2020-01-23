use deathframe::specs_physics;
use specs_physics::nphysics::object::{Body, ColliderDesc, Ground, RigidBody};

mod body_wrapper;
mod collider_wrapper;

pub use body_wrapper::prelude::*;
pub use collider_wrapper::prelude::*;

#[derive(Clone, Deserialize)]
pub struct PhysicsData {
    /// Data relevant to the body.
    pub body: BodyWrapper,

    /// Data relevant to the collider.
    pub collider: ColliderWrapper,
}

impl PhysicsData {
    pub fn body(&self) -> Box<dyn Body<f32>> {
        match &self.body {
            BodyWrapper::RigidBody(rigid_body_wrapper) => {
                Box::<RigidBody<f32>>::new(rigid_body_wrapper.into())
            }
            BodyWrapper::Ground(ground_wrapper) => {
                Box::<Ground<f32>>::new(ground_wrapper.into())
            }
        }
    }

    pub fn collider(&self) -> ColliderDesc<f32> {
        (&self.collider).into()
    }
}
