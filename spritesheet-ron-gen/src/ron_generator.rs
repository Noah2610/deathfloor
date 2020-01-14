use crate::{PngData, Size};
use std::fs;
use std::io::Write;
use std::path::Path;

// TODO: use tile size from command-line option
pub const DEFAULT_TILE_SIZE: (u32, u32) = (32, 32);

#[derive(Debug, Serialize)]
struct SpriteData {
    x:      u32,
    y:      u32,
    width:  u32,
    height: u32,
}

#[derive(Debug, Serialize)]
#[serde(rename = "SpriteList")]
struct SpritesheetData {
    texture_width:  u32,
    texture_height: u32,
    sprites:        Vec<SpriteData>,
}

impl SpritesheetData {
    pub fn gen_sprites_with_tile_size(&mut self, tile_size: Size) {
        self.sprites.clear();

        let cols = self.texture_width / tile_size.w;
        let rows = self.texture_height / tile_size.h;

        for row in 0 .. rows {
            for col in 0 .. cols {
                self.sprites.push(SpriteData {
                    x:      col * tile_size.w,
                    y:      row * tile_size.h,
                    width:  tile_size.w,
                    height: tile_size.h,
                });
            }
        }
    }
}

impl From<PngData> for SpritesheetData {
    fn from(png_data: PngData) -> Self {
        Self {
            texture_width:  png_data.size.w,
            texture_height: png_data.size.h,
            sprites:        Vec::new(),
        }
    }
}

// TODO: add command-line flag to set if this wrapper should be used or not
// reasoning for wrapper:
//     newer amethyst version needs this,
//     this will probably change in the future
#[derive(Debug, Serialize)]
struct RonWrapper(SpritesheetData);

pub struct GenerateOptions {
    pub tile_size: Size,
    pub pretty:    bool,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            tile_size: Size::from(DEFAULT_TILE_SIZE),
            pretty:    false,
        }
    }
}

pub fn generate_rons_for_pngs(
    pngs_data: Vec<PngData>,
    generate_options: GenerateOptions,
) -> Result<(), String> {
    for png_data in pngs_data {
        let ron_file_path = {
            let dir = png_data.path.parent().unwrap_or(Path::new("."));
            let name = png_data
                .path
                .file_stem()
                .ok_or_else(|| {
                    String::from("PngData's path should have file name")
                })?
                .to_str()
                .ok_or_else(|| String::from("Couldn't convert &OsStr to &str"))?
                .to_string()
                + ".ron";
            dir.join(name)
        };

        let mut spritesheet_data = SpritesheetData::from(png_data);
        spritesheet_data.gen_sprites_with_tile_size(generate_options.tile_size);

        // TODO: add command-line flag to set if this wrapper should be used or not
        let wrapper = RonWrapper(spritesheet_data);

        // TODO: add command-line flag for pretty/ugly RON formatting
        let ron_s = {
            let ser_err_fn =
                |e| format!("Couldn't serialize spritesheet data: {}", e);
            if generate_options.pretty {
                let pretty_config = ron::ser::PrettyConfig::default();
                ron::ser::to_string_pretty(&wrapper, pretty_config)
                    .map_err(ser_err_fn)?
            } else {
                ron::ser::to_string(&wrapper).map_err(ser_err_fn)?
            }
        };

        // TODO: add command-line option to set where to save generated
        //       RON file, relative to its PNG file
        let mut ron_file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&ron_file_path)
            .map_err(|e| {
                format!(
                    "Couldn't open RON file for writing: {:?}\n{}",
                    &ron_file_path, e
                )
            })?;
        ron_file.write_all(ron_s.as_bytes()).map_err(|e| {
            format!("Couldn't write to RON file: {:?}\n{}", &ron_file_path, e)
        })?;
        ron_file.flush().map_err(|e| {
            format!("Couldn't flush file: {:?}\n{}", &ron_file_path, e)
        })?;
    }

    Ok(())
}
