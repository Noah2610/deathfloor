#[derive(Clone, Deserialize)]
pub enum Action {
    Echo(String),
    Group(Vec<Action>),
    SetVelocity { x: Option<f32>, y: Option<f32> },
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Echo,
    Group,
    SetVelocity,
}

impl From<&Action> for ActionType {
    fn from(action: &Action) -> Self {
        match action {
            Action::Echo(_) => ActionType::Echo,
            Action::Group(_) => ActionType::Group,
            Action::SetVelocity { .. } => ActionType::SetVelocity,
        }
    }
}
