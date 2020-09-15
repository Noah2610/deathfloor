/// Call the `Function` event with the given name.
#[derive(Clone, Deserialize)]
#[serde(from = "String")]
pub struct Call(pub String);

impl From<String> for Call {
    fn from(s: String) -> Self {
        Self(s)
    }
}
