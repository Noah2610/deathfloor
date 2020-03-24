use amethyst::core::frame_limiter::FrameRateLimitConfig;
use deathframe::amethyst;

use crate::animation_key::AnimationKey;
use crate::collision_tag;
use crate::helpers::resource;
use crate::input;
use crate::states::prelude::*;

pub fn init_game() -> amethyst::Result<()> {
    use amethyst::utils::app_root_dir::application_root_dir;
    use amethyst::ApplicationBuilder;

    start_logger();

    let mut game: amethyst::CoreApplication<GameData> =
        ApplicationBuilder::new(application_root_dir()?, Startup::default())?
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

fn build_game_data<'a, 'b>() -> amethyst::Result<GameDataBuilder<'a, 'b>> {
    use crate::systems::prelude::*;
    use amethyst::core::transform::TransformBundle;
    use amethyst::renderer::types::DefaultBackend;
    use amethyst::renderer::{RenderFlat2D, RenderToWindow, RenderingBundle};
    use amethyst::utils::fps_counter::FpsCounterBundle;
    use deathframe::bundles::*;

    let transform_bundle = TransformBundle::new();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(resource("config/display.ron"))?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default());
    let ingame_input_bundle = input::ingame_input_bundle()?;
    let paused_input_bundle = input::paused_input_bundle()?;
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
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(rendering_bundle)?
        .with_core(ScaleSpritesSystem::default(), "scale_sprites_system", &[])?
        .with_core(CameraOrthoSystem::default(), "camera_ortho_system", &[])?
        .with_bundle(DispatcherId::Ingame, ingame_input_bundle)?
        .with_bundle(DispatcherId::Paused, paused_input_bundle)?
        .with_bundle(DispatcherId::Ingame, physics_bundle)?
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
            EntityLoaderSystem::default(),
            "entity_loader_system",
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
            CreateBulletsSystem::default(),
            "create_bullets_system",
            &["control_player_shoot_system"],
        )?
        .with(
            DispatcherId::Ingame,
            DeleteBulletsSystem::default(),
            "delete_bullets_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            BulletHitSystem::default(),
            "bullet_hit_system",
            &[
                "create_bullets_system",
                "delete_bullets_system",
                "update_collisions_system",
            ],
        )?
        .with(
            DispatcherId::Ingame,
            HandleWalkersSystem::default(),
            "handle_walkers_system",
            &[],
        )?
        .with(
            DispatcherId::Ingame,
            HandleEnemyActionsSystem::default(),
            "handle_enemy_actions_system",
            &[],
        )?;

    #[cfg(feature = "debug")]
    {
        const PRINT_EVERY_MS: u64 = 1000;
        let fps_bundle = FpsCounterBundle;
        let debug_system = DebugSystem::new(PRINT_EVERY_MS);

        custom_game_data = custom_game_data
            .with_core_bundle(fps_bundle)?
            .with_core(debug_system, "debug_system", &[])?;
    }

    Ok(custom_game_data)
}
