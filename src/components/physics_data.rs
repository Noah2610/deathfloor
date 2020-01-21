use super::component_prelude::*;
use crate::settings::SizeSettings;
use specs_physics::ncollide::shape::{Cuboid, ShapeHandle};
use specs_physics::nphysics::material::{BasicMaterial, MaterialHandle};
use specs_physics::nphysics::object::{
    BodyStatus,
    ColliderDesc,
    RigidBodyDesc,
};

type AccelerationSpeed = (Option<f32>, Option<f32>);

#[derive(Clone, Component, Deserialize)]
pub enum BodyStatusWrapper {
    Disabled,
    Static,
    Dynamic,
    Kinematic,
}

impl Into<BodyStatus> for &BodyStatusWrapper {
    fn into(self) -> BodyStatus {
        match self {
            BodyStatusWrapper::Disabled => BodyStatus::Disabled,
            BodyStatusWrapper::Static => BodyStatus::Static,
            BodyStatusWrapper::Dynamic => BodyStatus::Dynamic,
            BodyStatusWrapper::Kinematic => BodyStatus::Kinematic,
        }
    }
}

#[derive(Clone, Component, Deserialize)]
#[storage(DenseVecStorage)]
pub struct PhysicsData {
    /// Optional acceleration for x/y axes.
    /// Not used in physics objects, added to RigidBody as user data.
    /// For now, this is only used with the player in the ControlPlayerSystem.
    pub acceleration: AccelerationSpeed,

    /// Optional max linear velocity.
    pub max_velocity: Option<f32>,

    /// How much movement is damped.
    /// When moving with the acceleration above,
    /// the max speed reached is `acceleration / damping`.
    pub damping: f32,

    /// The body's mass.
    pub mass: f32,

    /// If gravity is enabled for this body.
    pub has_gravity: bool,

    /// See `specs_physics::nphysics::object::BodyStatus`.
    pub body_status: BodyStatusWrapper,

    /// The material's friction.
    pub friction: f32,

    /// The material's restitution.
    pub restitution: f32,
}

impl PhysicsData {
    pub fn rigid_body(&self) -> RigidBodyDesc<f32> {
        use std::f32;
        RigidBodyDesc::new()
            .gravity_enabled(self.has_gravity)
            .status((&self.body_status).into())
            .linear_damping(self.damping)
            .max_linear_velocity(self.max_velocity.unwrap_or(f32::MAX))
            .mass(self.mass)
            .user_data(self.user_data())
    }

    pub fn collider(&self, size: (f32, f32)) -> ColliderDesc<f32> {
        let shape = ShapeHandle::new(Cuboid::new(Vector::new(
            size.0 * 0.5,
            size.1 * 0.5,
        )));
        ColliderDesc::new(shape).material(self.material())
    }

    fn user_data(&self) -> PhysicsUserData {
        PhysicsUserData {
            acceleration: self.acceleration,
        }
    }

    fn material(&self) -> MaterialHandle<f32> {
        MaterialHandle::new(BasicMaterial::new(self.restitution, self.friction))
    }
}

#[derive(Debug, Clone)]
pub struct PhysicsUserData {
    pub acceleration: AccelerationSpeed,
}
