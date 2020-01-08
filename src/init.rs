use amethyst::core::frame_limiter::FrameRateLimitConfig;
use deathframe::amethyst;

use crate::helpers::resource;
use crate::states;

pub fn init_game() -> amethyst::Result<()> {
    use amethyst::utils::app_root_dir::application_root_dir;
    use amethyst::ApplicationBuilder;

    start_logger();

    let mut game: amethyst::CoreApplication<states::GameData> =
        ApplicationBuilder::new(
            application_root_dir()?,
            states::prelude::Startup::default(),
        )?
        .with_frame_limit_config(frame_rate_limit_config()?)
        .build(build_game_data()?)?;
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

fn frame_rate_limit_config() -> amethyst::Result<FrameRateLimitConfig> {
    use std::fs::File;
    Ok(ron::de::from_reader(File::open(resource(
        "config/frame_limiter.ron",
    ))?)?)
}

fn build_game_data<'a, 'b>() -> amethyst::Result<states::GameDataBuilder<'a, 'b>>
{
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use states::prelude::*;

    // Bundles
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(resource("config/display.ron"))?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());
    let transform_bundle = TransformBundle::new();

    let custom_game_data = GameDataBuilder::default()
        .custom(CustomData::default())
        .dispatcher(DispatcherId::Startup)?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(transform_bundle)?;

    Ok(custom_game_data)
}
