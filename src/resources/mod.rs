pub mod prelude {
    pub use super::object_spawner::{ObjectSpawnData, ObjectSpawner};
    pub use super::song_type::SongType;
    pub use super::sound_type::SoundType;
    pub use crate::settings::prelude::*;
    pub use deathframe::resources::prelude::*;
}

mod object_spawner;
mod song_type;
mod sound_type;
