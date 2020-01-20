mod physics_data;
mod player;

pub mod prelude {
    pub type RigidBody = specs_physics::nphysics::object::RigidBody<f32>;
    pub type Force = specs_physics::nphysics::algebra::Force2<f32>;
    pub type ReadRigidBodies<'a> = specs_physics::ReadRigidBodies<'a, f32>;
    pub type WriteRigidBodies<'a> = specs_physics::WriteRigidBodies<'a, f32>;
    pub type Velocity = specs_physics::nphysics::math::Velocity<f32>;

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

    pub use super::physics_data::PhysicsData;
    pub use super::player::Player;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::prelude::*;
}
