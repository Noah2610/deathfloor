pub fn resource<S>(path: S) -> String
where
    S: Into<String>,
{
    use deathframe::amethyst::utils::app_root_dir::application_dir;

    let path: String = path.into();

    let res_dir =
        application_dir("resources").expect("Should have resources directory");

    let path = res_dir.join(path);
    path.to_str().unwrap().to_string()
}
