use super::CTag;

#[derive(Clone, Hash, Deserialize)]
#[serde(from = "Option<Vec<T>>")]
pub struct EnemyCollidesWith<T: CTag>(pub Option<Vec<T>>);

/// `EnemyCollidesWith` are _always_ equal to each other.
impl<T> PartialEq for EnemyCollidesWith<T>
where
    T: CTag,
{
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> Eq for EnemyCollidesWith<T> where T: CTag
{
}

impl<T> From<Option<Vec<T>>> for EnemyCollidesWith<T>
where
    T: CTag,
{
    fn from(collides_with: Option<Vec<T>>) -> Self {
        Self(collides_with)
    }
}
