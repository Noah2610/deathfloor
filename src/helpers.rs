pub fn resource<S>(path: S) -> String
where
    S: Into<String>,
{
    use deathframe::amethyst::utils::app_root_dir::application_dir;

    let path = if cfg!(target_os = "windows") {
        path.into().replace("/", "\\")
    } else {
        path.into()
    };

    let res_dir =
        application_dir("resources").expect("Should have resources directory");

    let path = res_dir.join(path);
    path.to_str().unwrap().to_string()
}
