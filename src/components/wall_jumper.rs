use super::component_prelude::*;

/// Entities that can _wall jump_.
/// Requires `Jumper` component.
#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct WallJumper;
