/// `EntityAction`s are used for manipulating `EntityConfig` stuff.
/// For example, to switch used _variant_, use the `SwitchVariant` action.
/// You can also _delete_ this entity with the `DeleteEntity` action.
#[derive(Clone, Deserialize)]
pub enum EntityAction {
    /// Switch out the current variant for another.
    /// Replaces the last entity config in the stack.
    SwitchVariant(String),
    /// Push a new variant onto the entity config stack.
    PushVariant(String),
    /// Pop off the last entity config stack,
    /// drops back to the previous variant.
    PopVariant,
    DeleteEntity,
}
