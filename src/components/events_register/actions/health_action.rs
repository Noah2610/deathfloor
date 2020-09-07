use crate::components::prelude::HealthAction as HealthActionComp;

/// _Gain_ or _lose_ health depending on the `HealthAction`.
/// Entity needs the `Health` component for this to work.
#[derive(Clone, Deserialize)]
#[serde(from = "HealthActionComp")]
pub struct HealthAction(pub HealthActionComp);

impl From<HealthActionComp> for HealthAction {
    fn from(comp: HealthActionComp) -> Self {
        Self(comp)
    }
}
