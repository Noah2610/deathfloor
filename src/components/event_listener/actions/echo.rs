use std::fmt;

/// Prints the given string to stdout.
#[derive(Clone, Deserialize)]
#[serde(from = "String")]
pub struct Echo(pub String);

impl From<String> for Echo {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for Echo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "> {}", self.0)
    }
}
