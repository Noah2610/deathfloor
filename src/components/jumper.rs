use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Jumper {
    pub strength:         JumperStrength,
    pub kill_strength:    JumperStrength,
    pub min_velocity:     JumperStrength,
    pub gravity:          Gravity,
    #[serde(skip)]
    pub is_jumping:       bool,
    #[serde(skip)]
    pub did_jump:         bool,
    #[serde(skip)]
    pub original_gravity: Option<Gravity>,
}

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JumperStrength {
    #[serde(default)]
    pub x: Option<f32>,
    #[serde(default)]
    pub y: Option<f32>,
}

impl<'a> ByAxis for &'a JumperStrength {
    type Item = &'a Option<f32>;
    fn by_axis(self, axis: &Axis) -> Self::Item {
        match axis {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
}
