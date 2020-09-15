use super::ActionType;

/// Groups a list of `ActionType`s together.
/// When this action is triggered, it triggers _all_ of its actions.
#[derive(Clone, Deserialize)]
#[serde(from = "Vec<ActionType>")]
pub struct Group(pub Vec<ActionType>);

impl Group {
    /// Flattens out all nested `Group` actions.
    pub fn flattend(self) -> Self {
        self.0
            .into_iter()
            .fold(Vec::new(), |mut acc, action| {
                if let ActionType::Group(nested_group) = action {
                    acc.append(&mut nested_group.flattend().0);
                } else {
                    acc.push(action);
                }
                acc
            })
            .into()
    }
}

impl From<Vec<ActionType>> for Group {
    fn from(actions: Vec<ActionType>) -> Self {
        Self(actions)
    }
}
