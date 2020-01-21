use deathframe::specs_physics;
use specs_physics::ncollide::shape::ShapeHandle;
use specs_physics::nphysics::material::{BasicMaterial, MaterialHandle};
use specs_physics::nphysics::object::{ColliderDesc, RigidBodyDesc};

mod body_status_wrapper;
mod shape_wrapper;

pub use body_status_wrapper::BodyStatusWrapper;
pub use shape_wrapper::ShapeWrapper;

#[derive(Clone, Deserialize)]
pub struct PhysicsData {
    /// Optional max linear velocity.
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

    /// See `specs_physics::::ncollide::shape`.
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

impl PhysicsData {
    pub fn rigid_body(&self) -> RigidBodyDesc<f32> {
        use std::f32;
        RigidBodyDesc::new()
            .gravity_enabled(self.has_gravity)
            .status((&self.body_status).into())
            .linear_damping(self.damping)
            .max_linear_velocity(self.max_velocity.unwrap_or(f32::MAX))
            .mass(self.mass)
    }

    pub fn collider(&self) -> ColliderDesc<f32> {
        ColliderDesc::new(self.shape())
            .margin(self.solid_margin.unwrap_or(0.01))
            .material(self.material())
    }

    fn shape(&self) -> ShapeHandle<f32> {
        (&self.shape).into()
    }

    fn material(&self) -> MaterialHandle<f32> {
        MaterialHandle::new(BasicMaterial::new(self.restitution, self.friction))
    }
}
