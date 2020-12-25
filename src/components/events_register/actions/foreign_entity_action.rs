use super::ActionType;
use crate::map_loader::types::ObjectType;
use std::hash::Hash;

#[derive(Clone, Deserialize)]
pub struct ForeignEntityAction {
    pub selector: ForeignEntitySelector,
    pub action:   Box<ActionType>,
}

#[derive(Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct ForeignEntitySelector {
    #[serde(alias = "object")]
    pub object_type: ObjectType,
}
