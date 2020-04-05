pub mod prelude {
    pub use super::lifecycle_state::LifecycleState;
    pub use super::Lifecycle;
}

mod lifecycle_state;

use super::component_prelude::*;
use lifecycle_state::LifecycleState;

#[derive(Component, Default, Clone)]
#[storage(VecStorage)]
pub struct Lifecycle {
    pub state:     LifecycleState,
    prolong_count: usize,
}
