use super::CTag;
use super::CollisionLabel;
use crate::settings::prelude::CollisionTagData;

#[derive(Clone, Hash, Deserialize, Builder)]
#[serde(from = "CollisionLabel")]
#[builder(pattern = "owned")]
pub struct CollisionTag {
    pub labels:        Vec<CollisionLabel>,
    #[builder(default)]
    pub collides_with: Vec<CollisionLabel>,
}

impl CollisionTag {
    pub fn builder() -> CollisionTagBuilder {
        CollisionTagBuilder::default()
    }
}

impl CollisionTagBuilder {
    pub fn label(mut self, label: CollisionLabel) -> Self {
        self.labels = Some(vec![label]);
        self
    }
}

impl CTag for CollisionTag {
    fn collides_with(&self, other: &Self) -> bool {
        other
            .labels
            .iter()
            .any(|other_label| self.collides_with.contains(other_label))
    }
}

impl From<CollisionLabel> for CollisionTag {
    fn from(label: CollisionLabel) -> Self {
        Self {
            labels:        vec![label],
            collides_with: Default::default(),
        }
    }
}

impl From<CollisionLabel> for CollisionTagBuilder {
    fn from(label: CollisionLabel) -> Self {
        Self {
            labels:        Some(vec![label]),
            collides_with: None,
        }
    }
}

/// Equal, if any of both tags' labels are equal.
impl PartialEq for CollisionTag {
    fn eq(&self, other: &Self) -> bool {
        self.labels.iter().any(|label| other.labels.contains(label))
    }
}

impl Eq for CollisionTag {
}

impl From<CollisionTagData> for CollisionTag {
    fn from(data: CollisionTagData) -> Self {
        Self {
            labels:        data.labels,
            collides_with: data.collides_with.unwrap_or_default(),
        }
    }
}
