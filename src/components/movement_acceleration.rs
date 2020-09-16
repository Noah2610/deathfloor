use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct MovementAcceleration {
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
}

impl<'a> ByAxis for &'a MovementAcceleration {
    type Item = &'a Option<f32>;
    fn by_axis(self, axis: &Axis) -> Self::Item {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
}
