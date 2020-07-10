/// `EntityAction`s are used for manipulating `EntityConfig` stuff.
/// For example, to switch used _variant_, use the `SwitchVariant` action.
/// You can also _delete_ this entity with the `DeleteEntity` action.
#[derive(Clone, Deserialize)]
pub enum EntityAction {
    SwitchVariant(String),
    DeleteEntity,
}
