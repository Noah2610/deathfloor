use crate::{PngData, Size};

// TODO: use tile size from command-line option
const TILE_SIZE: (u32, u32) = (32, 32);

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

pub fn generate_rons_for_pngs(pngs_data: Vec<PngData>) -> Result<(), String> {
    for png_data in pngs_data {
        let mut spritesheet_data = SpritesheetData::from(png_data);
        spritesheet_data.gen_sprites_with_tile_size(Size::from(TILE_SIZE));

        // TODO: add command-line flag to set if this wrapper should be used or not
        let wrapper = RonWrapper(spritesheet_data);

        // TODO: add command-line flag for pretty/ugly RON formatting
        let pretty_config = ron::ser::PrettyConfig::default();
        let ron_s = ron::ser::to_string_pretty(&wrapper, pretty_config)
            .map_err(|e| {
                format!("Couldn't serialize spritesheet data: {}", e)
            })?;
        println!("{}", &ron_s);
    }

    unimplemented!()
}
