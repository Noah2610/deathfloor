use crate::components::prelude::MoveAction as MoveActionComp;

/// When triggers, queueus the given `MoveAction`.
#[derive(Clone, Deserialize)]
#[serde(from = "MoveActionComp")]
pub struct MoveAction(pub MoveActionComp);

impl From<MoveActionComp> for MoveAction {
    fn from(comp: MoveActionComp) -> Self {
        Self(comp)
    }
}
