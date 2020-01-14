use png::Decoder;
use std::convert::TryFrom;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

impl From<png::OutputInfo> for Size {
    fn from(info: png::OutputInfo) -> Self {
        Self {
            w: info.width,
            h: info.height,
        }
    }
}

#[derive(Debug)]
pub struct PngData {
    pub path: PathBuf,
    pub size: Size,
}

impl TryFrom<PathBuf> for PngData {
    type Error = String;

    fn try_from(path: PathBuf) -> Result<Self, Self::Error> {
        let file = File::open(&path).map_err(|e| {
            format!("Couldn't open file for reading: {:?}\n{}", path, e)
        })?;
        let decoder = Decoder::new(file);
        let info = decoder
            .read_info()
            .map_err(|e| {
                format!("Couldn't read PNG file's metadata: {:?}\n{}", path, e)
            })?
            .0;
        let size = Size::from(info);
        Ok(Self { path, size })
    }
}
