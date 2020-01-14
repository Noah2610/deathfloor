#[derive(Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
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
