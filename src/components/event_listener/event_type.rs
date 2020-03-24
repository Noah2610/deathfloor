#[derive(PartialEq, Eq, Hash, Clone, Deserialize)]
pub enum EventType {
    OnSpawn,
    OnDeath,
}
