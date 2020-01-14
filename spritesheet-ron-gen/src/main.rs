extern crate png;
extern crate ron;
#[macro_use]
extern crate serde;

use std::path::PathBuf;

mod action;
mod help;
mod meta;
mod png_data;

use action::Action;
use png_data::PngData;
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

fn generate_rons_for_pngs(png_data: Vec<PngData>) -> Result<(), String> {
    unimplemented!()
}
