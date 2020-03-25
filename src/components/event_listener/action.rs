#[derive(Clone, Deserialize)]
pub enum Action {
    Echo(String),
    Group(Vec<Action>),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Echo,
    Group,
}

impl From<&Action> for ActionType {
    fn from(action: &Action) -> Self {
        match action {
            Action::Echo(_) => ActionType::Echo,
            Action::Group(_) => ActionType::Group,
        }
    }
}
