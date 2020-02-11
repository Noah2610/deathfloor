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

    pub fn set(&mut self, axis: &Axis, max: f32) {
        match axis {
            Axis::X => self.x = Some(max),
            Axis::Y => self.y = Some(max),
        }
    }

    pub fn set_opt(&mut self, axis: &Axis, max: Option<f32>) {
        match axis {
            Axis::X => self.x = max,
            Axis::Y => self.y = max,
        }
    }
}

impl MaxMovementVelocityBuilder {
    pub fn with(mut self, axis: &Axis, max: f32) -> Self {
        match axis {
            Axis::X => self.x = Some(Some(max)),
            Axis::Y => self.y = Some(Some(max)),
        }
        self
    }

    pub fn with_opt(mut self, axis: &Axis, max: Option<f32>) -> Self {
        match axis {
            Axis::X => self.x = Some(max),
            Axis::Y => self.y = Some(max),
        }
        self
    }
}
