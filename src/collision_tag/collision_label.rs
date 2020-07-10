#[derive(Clone, PartialEq, Eq, Hash, Deserialize)]
#[serde(from = "String")]
pub struct CollisionLabel(pub String);

impl CollisionLabel {
    /// Returns the label we decided to make the "Tile" label.
    pub fn tile() -> Self {
        "Tile".into()
    }

    /// Returns the label we decided to make the "Solid" label.
    pub fn solid() -> Self {
        "Solid".into()
    }

    /// Returns the label we decided to make the "Bullet" label.
    pub fn bullet() -> Self {
        "Bullet".into()
    }
}

impl From<String> for CollisionLabel {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for CollisionLabel {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}
