#[derive(Deserialize, Clone)]
pub enum ControlAction {
    SetControllable(bool),
}
