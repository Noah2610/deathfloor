use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl Size {
    fn try_from_s<S>(s: S) -> Result<Self, String>
    where
        S: Into<String>,
    {
        const ERR_MSG: &str = r#"
SIZE string must have the format:
`<width>x<height>`
where <width> and <height> are positive integers."#;
        const DELIM: &str = "x";

        let s = s.into();
        let split = s.split(DELIM).collect::<Vec<&str>>();
        if split.len() != 2 {
            return Err(String::from(ERR_MSG));
        }

        Ok(Self {
            w: split[0].parse().or(Err(ERR_MSG))?,
            h: split[1].parse().or(Err(ERR_MSG))?,
        })
    }
}

impl From<(u32, u32)> for Size {
    fn from(tup: (u32, u32)) -> Self {
        Self { w: tup.0, h: tup.1 }
    }
}

impl From<png::OutputInfo> for Size {
    fn from(info: png::OutputInfo) -> Self {
        Self {
            w: info.width,
            h: info.height,
        }
    }
}

impl TryFrom<&str> for Size {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from_s(s)
    }
}

impl TryFrom<String> for Size {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        Self::try_from_s(s)
    }
}

impl FromStr for Size {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_s(s)
    }
}

impl Into<String> for Size {
    fn into(self) -> String {
        format!("{}x{}", self.w, self.h)
    }
}
