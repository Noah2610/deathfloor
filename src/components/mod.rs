pub mod grounded;
pub mod player;

pub mod prelude {
    pub type BodyComponent = specs_physics::bodies::BodyComponent<f32>;
    pub type Collider<T> = specs_physics::nphysics::object::Collider<f32, T>;
    pub type ColliderComponent =
        specs_physics::colliders::ColliderComponent<f32>;
    pub type ColliderSet<'f> = specs_physics::colliders::ColliderSet<'f, f32>;
    pub type Force = specs_physics::nphysics::algebra::Force2<f32>;
    pub type ReadRigidBodies<'a> = specs_physics::ReadRigidBodies<'a, f32>;
    pub type RigidBody = specs_physics::nphysics::object::RigidBody<f32>;
    pub type Velocity = specs_physics::nphysics::math::Velocity<f32>;
    pub type WriteRigidBodies<'a> = specs_physics::WriteRigidBodies<'a, f32>;

    pub use amethyst::renderer::SpriteRender;
    pub use deathframe::amethyst;
    pub use deathframe::components::prelude::*;
    pub use deathframe::specs_physics;
    pub use specs_physics::nphysics::algebra::ForceType;
    pub use specs_physics::nphysics::object::{
        Body as _,
        BodyPart as _,
        RigidBody as _,
    };

    pub use super::grounded::{Grounded, GroundedInitialized};
    pub use super::player::Player;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
