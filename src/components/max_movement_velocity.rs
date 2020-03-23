use super::component_prelude::*;

/// The maximum velocity a `Movable` entity can have, when being moved.
#[derive(Default, Component, Builder, Deserialize, Clone)]
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
        (self.x, self.y).by_axis(axis)
    }

    pub fn set(&mut self, axis: &Axis, max: f32) {
        *(&mut self.x, &mut self.y).by_axis(axis) = Some(max);
    }

    pub fn set_opt(&mut self, axis: &Axis, max: Option<f32>) {
        *(&mut self.x, &mut self.y).by_axis(axis) = max;
    }
}

impl MaxMovementVelocityBuilder {
    pub fn with(mut self, axis: &Axis, max: f32) -> Self {
        *(&mut self.x, &mut self.y).by_axis(axis) = Some(Some(max));
        self
    }

    pub fn with_opt(mut self, axis: &Axis, max: Option<f32>) -> Self {
        *(&mut self.x, &mut self.y).by_axis(axis) = Some(max);
        self
    }
}
