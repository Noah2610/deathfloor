use deathframe::amethyst::utils::app_root_dir::application_dir;
use std::path::{Path, PathBuf};

pub fn resource<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    application_dir("resources")
        .expect("Should have resources directory")
        .join(path)
}
