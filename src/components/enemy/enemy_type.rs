use std::fmt;

#[derive(Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(from = "String")]
pub struct EnemyType(pub String);

impl fmt::Display for EnemyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl From<String> for EnemyType {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for EnemyType {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}
