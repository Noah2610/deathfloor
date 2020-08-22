use crate::animation_key::AnimationKey;
use crate::collision_tag;
use crate::helpers::resource;
use crate::input;
use crate::settings::Settings;
use crate::states::prelude::*;
use amethyst::core::frame_limiter::FrameRateLimitConfig;
use amethyst::window::DisplayConfig;
use deathframe::amethyst;
use std::path::PathBuf;

pub fn init_game() -> amethyst::Result<()> {
    use crate::resources::prelude::*;
    use amethyst::utils::app_root_dir::application_root_dir;
    use amethyst::ApplicationBuilder;

    start_logger();

    let settings = Settings::load()?;
    let game_data = build_game_data(&settings)?;

    let mut game_builder =
        ApplicationBuilder::new(application_root_dir()?, Startup::default())?
            .with_frame_limit_config(frame_rate_limit_config()?)
            .with_resource(SpriteSheetHandles::<PathBuf>::default());

    #[cfg(feature = "debug")]
    {
        use amethyst::utils::fps_counter::FpsCounter;
        game_builder = game_builder.with_resource(FpsCounter::new(
            settings.general.debug.fps_sample_size,
        ));
    }

    let mut game: amethyst::CoreApplication<GameData> =
        game_builder.with_resource(settings).build(game_data)?;

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

fn get_display_config() -> amethyst::Result<DisplayConfig> {
    use std::fs::File;

    let file = File::open(resource("config/display.ron"))?;
    let display_config: DisplayConfig = {
        #[cfg(not(feature = "dev"))]
        {
            ron::de::from_reader(file)?
        }
        #[cfg(feature = "dev")]
        {
            let mut config: DisplayConfig = ron::de::from_reader(file)?;
            config.max_dimensions = config.dimensions.clone();
            config.min_dimensions = config.dimensions.clone();
            config
        }
    };

    Ok(display_config)
}

fn build_game_data<'a, 'b>(
    settings: &Settings,
) -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    use crate::resources::prelude::{SongType, SoundType};
    use crate::systems::prelude::*;
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use amethyst::ui::{RenderUi, UiBundle};
    use deathframe::bundles::*;

    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config(get_display_config()?)
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderUi::default())
        .with_plugin(RenderFlat2D::default());
    let audio_bundle = AudioBundle::<SoundType, SongType>::default()
        .with_sounds_default_volume(
            settings.general.audio.sounds_default_volume,
        );
    let ingame_input_bundle = input::ingame_input_bundle()?;
    let paused_input_bundle = input::paused_input_bundle()?;
    let menu_input_bundle = input::menu_input_bundle()?;
    let physics_bundle = PhysicsBundle::<
        collision_tag::CollisionTag,
        collision_tag::SolidTag,
    >::new()
    .with_deps(&[]);
    let animation_bundle = AnimationBundle::<AnimationKey>::new()
        .with_deps(&["handle_animations_system"]);

    let mut custom_game_data = GameDataBuilder::default()
        .custom(CustomData::default())
        .dispatcher(DispatcherId::Ingame)?
        .dispatcher(DispatcherId::Paused)?
        .dispatcher(DispatcherId::Ui)?
        .dispatcher(DispatcherId::MainMenu)?
        .dispatcher(DispatcherId::LevelSelect)?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(rendering_bundle)?
        .with_core_bundle(audio_bundle)?
        .with_core_bundle(menu_input_bundle)?
        .with_core_bundle(UiBundle::<input::MenuBindings>::new())?
        .with_core(ScaleSpritesSystem::default(), "scale_sprites_system", &[])?
        .with_core(CameraOrthoSystem::default(), "camera_ortho_system", &[])?
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
        .with_bundle(DispatcherId::Paused, paused_input_bundle)?
        .with(
            DispatcherId::Ui,
            InputManagerSystem::<input::MenuBindings>::default(),
            "ui_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::LevelSelect,
            HandleLevelSelectSystem::default(),
            "handle_level_select_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleAnimationsSystem::default(),
            "handle_animations_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleScalesSystem::default(),
            "handle_scales_system",
            &[],
        )?
        .with_bundle(DispatcherId::Ingame, animation_bundle)?
        .with(
            DispatcherId::Ingame,
            InputManagerSystem::<input::IngameBindings>::default(),
            "ingame_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Paused,
            InputManagerSystem::<input::PausedBindings>::default(),
            "paused_input_manager_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            FollowSystem::default(),
            "follow_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            ConfineEntitiesSystem::default(),
            "confine_entities_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            EntityLoaderSystem::default()
                .with_cache(settings.general.loader_system.use_cache),
            "entity_loader_system",
            &[
                "move_entities_system",
                "follow_system",
                "confine_entities_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayerSystem::default(),
            "control_player_system",
            &["ingame_input_manager_system"],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayerJumpSystem::default(),
            "control_player_jump_system",
            &["ingame_input_manager_system"],
        )?
        .with(
            DispatcherId::Ingame,
            ControlPlayerShootSystem::default(),
            "control_player_shoot_system",
            &["ingame_input_manager_system", "update_collisions_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleJumppadAffectedSystem::default(),
            "handle_jumppad_affected_system",
            &["update_collisions_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleMovablesSystem::default(),
            "handle_movables_system",
            &[
                "control_player_system",
                "control_player_jump_system",
                "handle_jumppad_affected_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            DeleteBulletsSystem::default(),
            "delete_bullets_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleWalkersSystem::default(),
            "handle_walkers_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            DisplayHealthSystem::default(),
            "display_health_system",
            &["move_entities_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateHealthSystem::default(),
            "update_health_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleAnimationEditorsSystem::default(),
            "handle_animation_editors_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateLifecycleSystem::default(),
            "update_lifecycle_system",
            &["update_health_system", "handle_taking_damage_system"],
        )?
        .with(
            DispatcherId::Ingame,
            UpdateEntityConfigsSystem::default(),
            "update_entity_configs_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            PlayDeathAnimationBeforeDeletionSystem::default(),
            "play_death_animation_before_deletion_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleLedgeDetectorSystem::default(),
            "handle_ledge_detector_system",
            &["update_collisions_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleDeathBoundSystem::default(),
            "handle_death_bound_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleDyingEntitiesSystem::default(),
            "handle_dying_entities_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleDeathOnContactSystem::default(),
            "handle_death_on_contact_system",
            &["update_collisions_system"],
        )?
        .with(
            DispatcherId::Ingame,
            HandleDeathAfterDelaySystem::default(),
            "handle_death_after_delay_system",
            &[],
        )?
        .with_bundle(DispatcherId::Ingame, EventHandlersBundle::default())?;

    #[cfg(feature = "debug")]
    {
        use amethyst::utils::fps_counter::FpsCounterBundle;
        use std::time::Duration;

        const PRINT_EVERY_MS: u64 = 1000;
        let fps_bundle = FpsCounterBundle;

        custom_game_data =
            custom_game_data.with_core_bundle(fps_bundle)?.with_core(
                PrintFpsSystem::default()
                    .with_print_delay(Duration::from_millis(PRINT_EVERY_MS)),
                "print_fps_system",
                &[],
            )?;
    }

    Ok(custom_game_data)
}
