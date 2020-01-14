extern crate png;
extern crate ron;
#[macro_use]
extern crate serde;
extern crate structopt;

mod action;
mod meta;
mod opts;
mod png_data;
mod ron_generator;
mod size;

use action::Action;
use png_data::PngData;
use size::Size;
use std::convert::TryFrom;
use std::path::PathBuf;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error:\n{}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let action = Action::new()?;
    run_action(action)
}

fn run_action(action: Action) -> Result<(), String> {
    match action {
        Action::Gen(files, generate_options) => {
            let png_data = get_png_info(files)?;
            ron_generator::generate_rons_for_pngs(png_data, generate_options)
        }
        Action::Help => print_help(),
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

fn print_help() -> Result<(), String> {
    use structopt::StructOpt;

    opts::Opts::clap()
        .print_help()
        .map_err(|e| format!("Couldn't print clap help\n{}", e))?;
    std::process::exit(1);
}
