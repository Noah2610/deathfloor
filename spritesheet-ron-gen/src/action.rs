use std::env;
use std::path::PathBuf;

pub enum Action {
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
