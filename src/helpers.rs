use deathframe::amethyst::utils::app_root_dir::application_dir;
use std::path::PathBuf;

pub fn resource<P>(path: P) -> PathBuf
where
    P: Into<PathBuf>,
{
    application_dir("resources")
        .expect("Should have resources directory")
        .join(
            if cfg!(target_os = "windows") {
                path.into().to_str().unwrap().replace("/", "\\").into()
            } else {
                path.into()
            },
        )
}
