extern crate png;

use std::path::PathBuf;

mod action;
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
            let files_info = get_files_info(files)?;
            dbg!(&files_info);
            // generate_rons_for_files(files)?;
            Ok(())
        }
        Action::Help => Ok(print_help()),
    }
}

fn get_files_info(paths: Vec<PathBuf>) -> Result<Vec<PngData>, String> {
    let mut files_data = Vec::new();

    for path in paths {
        if path.is_file() {
            files_data.push(PngData::try_from(path)?);
        } else {
            return Err(format!("File doesn't exist: {:?}", path));
        }
    }

    Ok(files_data)
}

fn print_help() {
    let help_txt = format!(
        r#"{NAME} v{VERSION}
{DESCRIPTION}

USAGE
    {EXECUTABLE} [OPTIONS] <FILES...>
    {EXECUTABLE} --help

ARGUMENTS
    FILES
        List of file paths to PNG images,
        for which to generate RON files.

OPTIONS
    --help, -h
        Print this help text and exit."#,
        NAME = meta::NAME,
        VERSION = meta::VERSION,
        DESCRIPTION = meta::DESCRIPTION,
        EXECUTABLE = meta::NAME,
    );

    println!("{}", help_txt);

    std::process::exit(0);
}
