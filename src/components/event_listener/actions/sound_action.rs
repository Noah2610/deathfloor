use crate::components::prelude::SoundAction as SoundActionComp;

/// When triggered, queues the given `SoundAction`.
#[derive(Clone, Deserialize)]
#[serde(from = "SoundActionComp")]
pub struct SoundAction(pub SoundActionComp);

impl From<SoundActionComp> for SoundAction {
    fn from(comp: SoundActionComp) -> Self {
        Self(comp)
    }
}
