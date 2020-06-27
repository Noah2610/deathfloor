use crate::components::prelude::EntityConfigRegisterAction;

/// `EntityAction`s are used for manipulating `EntityConfig` stuff.
/// For example, to switch used _variant_, use the `SwitchVariant` action.
#[derive(Clone, Deserialize)]
#[serde(from = "EntityConfigRegisterAction")]
pub struct EntityAction(pub EntityConfigRegisterAction);

impl From<EntityConfigRegisterAction> for EntityAction {
    fn from(action: EntityConfigRegisterAction) -> Self {
        Self(action)
    }
}
