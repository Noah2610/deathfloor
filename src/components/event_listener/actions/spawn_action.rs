use crate::resources::prelude::ObjectSpawnData;

#[derive(Clone, Deserialize)]
pub enum SpawnAction {
    SpawnAbsolute(ObjectSpawnData),
    SpawnRelative(ObjectSpawnData),
}
