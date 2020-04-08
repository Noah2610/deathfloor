use super::CTag;
use super::CollisionLabel;

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
        self.collides_with.iter().any(|collides_with_label| {
            other.labels.contains(collides_with_label)
        })
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

impl From<Vec<CollisionLabel>> for CollisionTag {
    fn from(labels: Vec<CollisionLabel>) -> Self {
        Self {
            labels:        labels,
            collides_with: Default::default(),
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

/// Wrapper for `CollisionTag`, used for deserialization.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CollisionTagWrapper {
    pub labels:        Vec<CollisionLabel>,
    pub collides_with: Option<Vec<CollisionLabel>>,
}

impl From<CollisionTagWrapper> for CollisionTag {
    fn from(data: CollisionTagWrapper) -> Self {
        Self {
            labels:        data.labels,
            collides_with: data.collides_with.unwrap_or_default(),
        }
    }
}
