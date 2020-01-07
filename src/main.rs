extern crate deathframe;

mod components;
mod helpers;
mod states;
mod systems;

use deathframe::amethyst;
use deathframe::custom_game_data::prelude::*;

fn main() {
    if let Err(e) = init_game() {
        eprintln!("An error occured: {}", e);
        std::process::exit(1);
    }
}

fn init_game() -> amethyst::Result<()> {
    use amethyst::utils::app_root_dir::application_root_dir;
    use amethyst::ApplicationBuilder;
    use helpers::resource;

    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<states::GameData> =
        ApplicationBuilder::new(
            application_root_dir().unwrap(),
            states::prelude::Startup::default(),
        )?
        // .with_frame_limit_config(FrameRateLimitConfig::load(resource(
        //     "config/frame_limiter.ron",
        // )))
        .build(game_data)?;
    game.run();

    Ok(())
}

fn start_logger() {
    use amethyst::{LogLevelFilter, LoggerConfig};

    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn build_game_data<'a, 'b>() -> amethyst::Result<states::GameDataBuilder<'a, 'b>>
{
    unimplemented!()
}
