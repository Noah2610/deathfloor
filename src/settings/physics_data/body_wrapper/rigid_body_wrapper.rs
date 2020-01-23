use super::BodyStatusWrapper;
use deathframe::specs_physics;
use specs_physics::nphysics::object::{RigidBody, RigidBodyDesc};

#[derive(Clone, Deserialize)]
pub struct RigidBodyWrapper {
    /// Max linear velocity.
    #[serde(default)]
    pub max_velocity: Option<f32>,

    /// How much movement is damped.
    /// When moving with the acceleration above,
    /// the max speed reached is `acceleration / damping`.
    #[serde(default)]
    pub damping: f32,

    /// The body's mass.
    #[serde(default)]
    pub mass: f32,

    /// If gravity is enabled for this body.
    #[serde(default)]
    pub has_gravity: bool,

    /// See `specs_physics::nphysics::object::BodyStatus`.
    pub body_status: BodyStatusWrapper,
}

impl Into<RigidBody<f32>> for &RigidBodyWrapper {
    fn into(self) -> RigidBody<f32> {
        use std::f32;
        RigidBodyDesc::new()
            .gravity_enabled(self.has_gravity)
            .status((&self.body_status).into())
            .linear_damping(self.damping)
            .max_linear_velocity(self.max_velocity.unwrap_or(f32::MAX))
            .mass(self.mass)
            .build()
    }
}
