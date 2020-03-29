use crate::components::prelude::MoveAction;

#[derive(Clone, Deserialize)]
pub enum Action {
    Echo(String),
    Group(Vec<Action>),
    MoveAction(MoveAction),
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Echo,
    Group,
    MoveAction,
}

impl From<&Action> for ActionType {
    fn from(action: &Action) -> Self {
        match action {
            Action::Echo(_) => ActionType::Echo,
            Action::Group(_) => ActionType::Group,
            Action::MoveAction(_) => ActionType::MoveAction,
        }
    }
}
