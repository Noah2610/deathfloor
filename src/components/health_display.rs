pub mod prelude {
    pub use super::HealthDisplay;
    pub use super::HealthDisplayPosition;
}

use super::component_prelude::*;

#[derive(Component, Clone, Deserialize)]
pub enum HealthDisplayPosition {
    Top,
    Bottom,
}

#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct HealthDisplay {
    pub position:       HealthDisplayPosition,
    pub size:           (f32, f32),
    pub padding:        f32,
    pub border_padding: f32,
}
