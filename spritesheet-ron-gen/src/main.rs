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

    match action {
        Action::Gen(files) => {}
        Action::Help => print_help(),
    }
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
