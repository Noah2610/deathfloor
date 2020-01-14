extern crate png;
extern crate ron;
#[macro_use]
extern crate serde;

use std::path::PathBuf;

mod action;
mod help;
mod meta;
mod png_data;
mod size;

use action::Action;
use png_data::PngData;
use size::Size;
use std::convert::TryFrom;

fn main() {
    let action = Action::current();
    if let Err(e) = run_action(action) {
        eprintln!("Error:\n{}", e);
        std::process::exit(1);
    }
}

fn run_action(action: Action) -> Result<(), String> {
    match action {
        Action::Gen(files) => {
            let png_data = get_png_info(files)?;
            generate_rons_for_pngs(png_data)?;
            Ok(())
        }
        Action::Help => Ok(help::print_help()),
    }
}

fn get_png_info(paths: Vec<PathBuf>) -> Result<Vec<PngData>, String> {
    paths.into_iter().try_fold(Vec::new(), |mut data, path| {
        if path.is_file() {
            data.push(PngData::try_from(path)?);
            Ok(data)
        } else {
            Err(format!("File doesn't exist: {:?}", path))
        }
    })
}

fn generate_rons_for_pngs(pngs_data: Vec<PngData>) -> Result<(), String> {
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

    for png_data in pngs_data {
        let mut spritesheet_data = SpritesheetData::from(png_data);
        spritesheet_data.gen_sprites_with_tile_size(Size::from(TILE_SIZE));

        // TODO: add command-line flag for pretty/ugly RON formatting

        let pretty_config = ron::ser::PrettyConfig::default();
        let ron_s =
            dbg!(ron::ser::to_string_pretty(&spritesheet_data, pretty_config));
    }

    unimplemented!()
}
