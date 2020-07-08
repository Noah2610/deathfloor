#[derive(Hash, PartialEq, Eq, Debug)]
pub enum DispatcherId {
    Ingame,
    Paused,
    Ui,
    MainMenu,
    LevelSelect,
}
