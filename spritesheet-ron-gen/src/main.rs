extern crate png;

use std::convert::TryFrom;
use std::env;
use std::path::PathBuf;

mod meta {
    pub const NAME: &str = env!("CARGO_PKG_NAME");
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
}

enum Action {
    Gen(Vec<PathBuf>),
    Help,
}

impl Action {
    pub fn current() -> Self {
        const HELP_OPTS: [&str; 3] = ["help", "--help", "-h"];

        let mut files = Vec::new();

        for arg in env::args().skip(1) {
            let s = arg.as_str();
            if HELP_OPTS.contains(&s) {
                return Action::Help;
            }

            let file = PathBuf::from(s);
            files.push(file);
        }

        if files.is_empty() {
            Self::default()
        } else {
            Action::Gen(files)
        }
    }
}

impl Default for Action {
    fn default() -> Self {
        Action::Help
    }
}

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
            let files_info = get_files_info(&files)?;
            // generate_rons_for_files(files)?;
            Ok(())
        }
        Action::Help => Ok(print_help()),
    }
}

fn get_files_info(
    paths: &Vec<PathBuf>,
) -> Result<Vec<png::OutputInfo>, String> {
    use png::Decoder;
    use std::fs::File;

    let mut files_info = Vec::new();

    for path in paths {
        if path.is_file() {
            let file = File::open(path).map_err(|e| {
                format!("Couldn't open file for reading: {:?}\n{}", path, e)
            })?;
            let decoder = Decoder::new(file);
            let info = decoder
                .read_info()
                .map_err(|e| {
                    format!(
                        "Couldn't read PNG file's metadata: {:?}\n{}",
                        path, e
                    )
                })?
                .0;
            files_info.push(info);
        } else {
            return Err(format!("File doesn't exist: {:?}", path));
        }
    }

    Ok(files_info)
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
