use crate::components::prelude::EntityConfigRegisterAction;

#[derive(Clone, Deserialize)]
#[serde(from = "EntityConfigRegisterAction")]
pub struct EntityAction(pub EntityConfigRegisterAction);

impl From<EntityConfigRegisterAction> for EntityAction {
    fn from(action: EntityConfigRegisterAction) -> Self {
        Self(action)
    }
}
