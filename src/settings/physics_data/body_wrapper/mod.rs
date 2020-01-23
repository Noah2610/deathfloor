mod body_status_wrapper;
mod ground_wrapper;
mod rigid_body_wrapper;

pub mod prelude {
    pub use super::body_status_wrapper::BodyStatusWrapper;
    pub use super::ground_wrapper::GroundWrapper;
    pub use super::rigid_body_wrapper::RigidBodyWrapper;
    pub use super::BodyWrapper;
}

use prelude::*;

#[derive(Clone, Deserialize)]
pub enum BodyWrapper {
    RigidBody(RigidBodyWrapper),
    Ground(GroundWrapper),
}
