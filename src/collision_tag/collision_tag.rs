use super::CTag;
use super::CollisionLabel;

#[derive(Clone, Hash, Deserialize, Builder)]
#[serde(from = "CollisionLabel")]
#[builder(pattern = "owned")]
pub struct CollisionTag {
    pub label:         CollisionLabel,
    #[builder(default)]
    pub collides_with: Vec<CollisionLabel>,
}

impl CollisionTag {
    pub fn builder() -> CollisionTagBuilder {
        CollisionTagBuilder::default()
    }
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        self.collides_with
            .iter()
            .any(|collides_with_label| collides_with_label == &other.label)
    }
}

impl From<CollisionLabel> for CollisionTag {
    fn from(label: CollisionLabel) -> Self {
        Self {
            label,
            collides_with: Default::default(),
        }
    }
}

impl From<CollisionLabel> for CollisionTagBuilder {
    fn from(label: CollisionLabel) -> Self {
        Self {
            label:         Some(label),
            collides_with: None,
        }
    }
}

/// `PartialEq` simply compares the two `CollisionTag`s labels.
impl PartialEq for CollisionTag {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for CollisionTag {
}
