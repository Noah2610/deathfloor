use crate::components::prelude::AnimationAction as AnimationActionComp;

/// _Play_, _push_, or _pop_ an animation.
/// See the `AnimationAction`'s documentation for details.
#[derive(Clone, Deserialize)]
#[serde(from = "AnimationActionComp")]
pub struct AnimationAction(pub AnimationActionComp);

impl From<AnimationActionComp> for AnimationAction {
    fn from(comp: AnimationActionComp) -> Self {
        Self(comp)
    }
}
