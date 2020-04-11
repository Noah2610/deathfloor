use crate::components::prelude::SoundAction as SoundActionComp;
use crate::resources::prelude::SoundType;

/// When triggered, queues the given `SoundAction`.
#[derive(Clone, Deserialize)]
#[serde(from = "SoundActionComp<SoundType>")]
pub struct SoundAction(pub SoundActionComp<SoundType>);

impl From<SoundActionComp<SoundType>> for SoundAction {
    fn from(comp: SoundActionComp<SoundType>) -> Self {
        Self(comp)
    }
}
