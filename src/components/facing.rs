use super::component_prelude::*;

#[derive(Component, PartialEq, Clone, Deserialize)]
#[storage(VecStorage)]
pub enum Facing {
    Left,
    Right,
}

impl Default for Facing {
    fn default() -> Self {
        Self::Right
    }
}

impl From<f32> for Facing {
    fn from(f: f32) -> Self {
        let sign = f.signum();
        if sign > 0.0 {
            Self::Right
        } else if sign < 0.0 {
            Self::Left
        } else {
            Self::default()
        }
    }
}
