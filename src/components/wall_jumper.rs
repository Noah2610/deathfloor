use super::component_prelude::*;
use super::jumper::JumperStrength;

/// Entities that can _wall jump_.
/// Requires `Jumper` component.
#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct WallJumper {
    pub strength: JumperStrength,
}
