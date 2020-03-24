#[derive(Clone, Deserialize)]
pub enum Action {
    Echo(String),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Echo,
}

impl From<&Action> for ActionType {
    fn from(action: &Action) -> Self {
        match action {
            Action::Echo(_) => ActionType::Echo,
        }
    }
}
