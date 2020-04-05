use super::component_prelude::*;

/// Entities that can _wall jump_.
/// Requires `Jumper` component.
#[derive(Component, Clone, Deserialize)]
#[storage(VecStorage)]
pub struct WallJumper {
    pub strength: (Option<f32>, Option<f32>),
}
