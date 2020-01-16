use deathframe::amethyst;
use std::convert::TryFrom;

#[derive(Debug)]
pub(super) enum ObjectType {
    Player,
}

impl ObjectType {
    fn try_from_s<S>(s: S) -> amethyst::Result<Self>
    where
        S: Into<String>,
    {
        let s = s.into();
        match s.as_str() {
            "Player" => Ok(ObjectType::Player),
            t => Err(amethyst::Error::from_string(format!(
                "Invalid object type: {}",
                t
            ))),
        }
    }
}

impl TryFrom<&str> for ObjectType {
    type Error = amethyst::Error;
    fn try_from(s: &str) -> amethyst::Result<Self> {
        Self::try_from_s(s)
    }
}

impl TryFrom<String> for ObjectType {
    type Error = amethyst::Error;
    fn try_from(s: String) -> amethyst::Result<Self> {
        Self::try_from_s(s)
    }
}
