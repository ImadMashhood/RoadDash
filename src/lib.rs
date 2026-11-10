pub mod contact_telemetry;
pub mod online;
mod police;

pub mod animation;
pub mod biomechanics;
pub mod combat;
pub mod content;
pub mod controls;
pub mod crash_behavior;
pub mod destruction;
pub mod diagnostics;
pub mod environment;
pub mod feedback;
pub mod frontend;
pub mod game;
pub mod human;
pub mod material_detail;
pub mod models;
pub mod npc;
pub mod offscreen_crash;
pub mod original_crash;
pub mod props;
pub mod recovery;
pub mod road_detail;
pub mod runtime;
pub mod settings;
pub mod sky;
pub mod sound;
pub mod tangent_detail;
pub mod track;
pub mod ui;
pub mod vehicle;
pub mod verification;
pub mod world;
pub mod world_gen;

use bevy::prelude::*;

/// Native offline client. Pure core/content crates can be tested without it.
pub fn build_app(smoke: bool) -> App {
    // Bevy's default IO workers inherit the OS stack, which overflows in the
    // debug glTF loader for the larger authored scenes. Configure only this
    // pool before DefaultPlugins; preserve Bevy's default 25%, 1..4 IO policy.
    bevy::tasks::IoTaskPool::get_or_init(|| {
        let threads = (bevy::tasks::available_parallelism() as f32 * 0.25).round() as usize;
        bevy::tasks::TaskPoolBuilder::new()
            .num_threads(threads.clamp(1, 4))
            .stack_size(16 * 1024 * 1024)
            .thread_name("RoadDash Asset IO".into())
            .build()
    });
    let mut app = App::new();
    app.insert_resource(original_crash::OriginalCrashes(
        std::env::args().any(|a| a == "--original-crashes"),
    ));

    app.insert_resource(
        biomechanics::Settings::selected().expect("valid ragdoll arguments/configuration"),
    );
    biomechanics::install_debug(&mut app);
    if !smoke {
        app.insert_resource(settings::Preferences::load(settings::Preferences::path()));
    }
    app.insert_resource(track::Track::selected());
    app.insert_resource(
        if std::env::var("ROADDASH_VEHICLE").as_deref() == Ok("assisted") {
            vehicle::VehiclePhysicsAdapter::Assisted
        } else {
            vehicle::VehiclePhysicsAdapter::Dynamic
        },
    );
    let window_title = format!(
        "ROAD / DASH — {}",
        app.world().resource::<track::Track>().definition.name
    );
    world_gen::city::register_assets(&mut app);
    props::register_assets(&mut app);
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                file_path: models::asset_directory(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: window_title,
                    mode: if std::env::args().any(|a| a == "--world-survey") {
                        bevy::window::WindowMode::BorderlessFullscreen(
                            bevy::window::MonitorSelection::Current,
                        )
                    } else {
                        bevy::window::WindowMode::Windowed
                    },
                    present_mode: if std::env::args().any(|a| a == "--world-survey") {
                        bevy::window::PresentMode::AutoNoVsync
                    } else {
                        default()
                    },
                    resolution: if std::env::args().any(|a| a == "--world-survey") {
                        (1920, 1080).into()
                    } else {
                        (1440, 900).into()
                    },
                    ..default()
                }),
                ..default()
            }),
    )
    .add_plugins(MaterialPlugin::<
        world_gen::metropolis::material::FacadeMaterial,
    >::default())
    .add_plugins(MaterialPlugin::<
        world_gen::metropolis::geometry::grass_material::GrassMaterial,
    >::default())
    .add_plugins(world_gen::city::visuals::BuildingSceneTypesPlugin)
    .add_plugins((
        content::RoadContentPlugin,
        runtime::RoadLifecyclePlugin,
        runtime::RoadSimulationPlugin,
        controls::RoadInputPlugin,
        models::RoadModelPlugin,
        props::PropPlugin,
        destruction::PropDestructionPlugin,
        world_gen::metropolis::geometry::crops::CropPlugin,
        runtime::RoadWorldPlugin,
        runtime::RoadPresentationPlugin,
        frontend::FrontendPlugin,
        online::OnlinePlugin,
        runtime::RoadAudioPlugin,
        diagnostics::RoadDiagnosticsPlugin,
    ));
    crash_behavior::install_comparison(&mut app);
    #[cfg(feature = "debug_physics")]
    app.add_plugins(bevy_rapier3d::render::RapierDebugRenderPlugin::default());
    if smoke {
        std::fs::create_dir_all("screenshots").expect("create screenshot directory");
        app.insert_resource(game::Persistence(false))
            .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            ));
        if std::env::args().any(|a| a == "--frontend-smoke") {
            frontend::install_smoke(&mut app);
        } else if std::env::args().any(|a| a == "--police-smoke") {
            app.init_resource::<police::survey::Survey>()
                .add_systems(
                    FixedUpdate,
                    police::survey::observe.after(runtime::SimSet::AfterPhysics),
                )
                .add_systems(Update, police::survey::run.after(ui::update_ui));
        } else if std::env::args().any(|a| a == "--destruction-survey") {
            app.insert_resource(bevy::winit::WinitSettings::continuous())
                .init_resource::<destruction::survey::Survey>()
                .add_systems(
                    Update,
                    destruction::survey::step.after(destruction::PropDamageVisuals),
                )
                .add_systems(
                    PostUpdate,
                    destruction::survey::camera
                        .after(game::camera)
                        .before(bevy::transform::TransformSystems::Propagate),
                );
        } else if app.world().resource::<track::Track>().metropolis.is_some()
            && std::env::args().any(|a| {
                matches!(
                    a.as_str(),
                    "--metro-survey" | "--world-survey" | "--lod-survey"
                )
            })
        {
            app.insert_resource(bevy::winit::WinitSettings::continuous())
                .init_resource::<world_gen::metropolis::survey::Survey>()
                .add_systems(
                    PostUpdate,
                    world_gen::metropolis::survey::run
                        .after(game::camera)
                        .before(bevy::transform::TransformSystems::Propagate),
                );
        } else if std::env::args().any(|a| a == "--lod-survey") {
            app.insert_resource(bevy::winit::WinitSettings::continuous())
                .init_resource::<world_gen::lod_survey::Survey>()
                .add_systems(
                    PostUpdate,
                    world_gen::lod_survey::run
                        .after(game::camera)
                        .before(bevy::transform::TransformSystems::Propagate),
                );
        } else if std::env::args().any(|a| a == "--world-survey") {
            app.insert_resource(bevy::winit::WinitSettings::continuous());
            if app
                .world()
                .resource::<track::Track>()
                .city_surface
                .is_some()
            {
                app.init_resource::<world_gen::city::survey::Survey>()
                    .add_systems(
                        PostUpdate,
                        world_gen::city::survey::run
                            .after(game::camera)
                            .before(bevy::transform::TransformSystems::Propagate),
                    );
            } else {
                app.init_resource::<world_gen::survey::Survey>()
                    .add_systems(
                        PostUpdate,
                        world_gen::survey::run
                            .after(game::camera)
                            .before(bevy::transform::TransformSystems::Propagate),
                    );
            }
        } else if std::env::args().any(|a| a == "--actor-skin-survey") {
            app.init_resource::<verification::ActorSkinSurvey>()
                .add_systems(Update, verification::actor_skin_survey.after(ui::update_ui))
                .add_systems(Last, verification::actor_skin_snapshot)
                .add_systems(
                    PostUpdate,
                    verification::actor_skin_camera
                        .after(game::camera)
                        .before(bevy::transform::TransformSystems::Propagate),
                );
        } else if std::env::args().any(|a| a == "--actor-smoke-test") {
            app.add_systems(Update, verification::actor_smoke.after(ui::update_ui));
        } else {
            app.init_resource::<verification::SmokeProgress>()
                .add_systems(
                    FixedUpdate,
                    (
                        verification::observe_smoke_recovery.after(recovery::recover),
                        verification::observe_smoke_strike.after(combat::melee),
                    ),
                )
                .add_systems(Update, verification::smoke_test.after(ui::update_ui));
        }
    }
    app
}

#[cfg(test)]
mod collision_tests;

pub mod weapons;

pub mod weapon_models;
