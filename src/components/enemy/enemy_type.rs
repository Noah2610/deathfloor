use std::fmt;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum EnemyType {
    Normal,
}

impl fmt::Display for EnemyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EnemyType::Normal => "Normal",
        })
    }
}
