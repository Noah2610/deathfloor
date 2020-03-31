use super::CTag;
use super::CollisionLabel;

#[derive(Clone, Hash, Deserialize)]
#[serde(from = "Option<Vec<CollisionLabel>>")]
pub struct EnemyCollidesWith(pub Vec<CollisionLabel>);

impl From<Option<Vec<CollisionLabel>>> for EnemyCollidesWith {
    fn from(collides_with: Option<Vec<CollisionLabel>>) -> Self {
        Self(collides_with.unwrap_or_default())
    }
}
