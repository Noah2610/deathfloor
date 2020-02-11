use super::component_prelude::*;

/// The maximum velocity a `Movable` entity can have, when being moved.
#[derive(Default, Component, Builder)]
#[storage(VecStorage)]
#[builder(pattern = "owned", default, setter(strip_option))]
pub struct MaxMovementVelocity {
    x: Option<f32>,
    y: Option<f32>,
}

impl MaxMovementVelocity {
    pub fn builder() -> MaxMovementVelocityBuilder {
        MaxMovementVelocityBuilder::default()
    }

    pub fn get(&self, axis: &Axis) -> Option<f32> {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    }
}
