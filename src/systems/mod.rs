mod check_grounded;
mod control_player;

pub mod prelude {
    pub use deathframe::amethyst::utils::ortho_camera::CameraOrthoSystem;
    pub use deathframe::systems::prelude::*;

    pub use super::check_grounded::CheckGroundedSystem;
    pub use super::control_player::ControlPlayerSystem;
}

mod system_prelude {
    pub use deathframe::systems::system_prelude::*;
    pub use deathframe::{amethyst, specs_physics};
    pub use specs_physics::{GeometricalWorldRes, MechanicalWorldRes};

    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::prelude::*;
}
