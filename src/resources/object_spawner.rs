use crate::map_loader::map_data::Object;

#[derive(Clone, Deserialize)]
pub struct ObjectSpawnData {
    pub object: Object,
}

#[derive(Default)]
pub struct ObjectSpawner {
    to_spawn: Vec<ObjectSpawnData>,
}

impl ObjectSpawner {
    pub fn add(&mut self, spawn_data: ObjectSpawnData) {
        self.to_spawn.push(spawn_data);
    }

    pub fn drain(&mut self) -> std::vec::Drain<ObjectSpawnData> {
        self.to_spawn.drain(..)
    }
}
