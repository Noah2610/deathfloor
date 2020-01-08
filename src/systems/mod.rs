mod control_player;

pub mod prelude {
    pub use deathframe::amethyst::utils::ortho_camera::CameraOrthoSystem;
    pub use deathframe::systems::prelude::*;

    pub use super::control_player::ControlPlayerSystem;
}

mod system_prelude {
    pub use deathframe::amethyst;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
}
