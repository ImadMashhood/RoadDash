#[cfg(not(feature = "offline"))]
compile_error!("No online transport is implemented. Enable the offline feature (default).");

pub mod content;
pub mod controls;
pub mod diagnostics;
pub mod environment;
pub mod game;
pub mod recovery;
pub mod runtime;
pub mod sound;
pub mod track;
pub mod ui;
pub mod verification;
pub mod world;

use bevy::prelude::*;

/// Native offline client. Pure core/content crates can be tested without it.
pub fn build_app(smoke: bool) -> App {
    let mut app = App::new();
    app.insert_resource(track::Track::selected());
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ROAD / DASH — Valley Run".into(),
            resolution: (1440, 900).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        content::RoadContentPlugin,
        runtime::RoadLifecyclePlugin,
        runtime::RoadSimulationPlugin,
        controls::RoadInputPlugin,
        runtime::RoadWorldPlugin,
        runtime::RoadPresentationPlugin,
        runtime::RoadAudioPlugin,
        diagnostics::RoadDiagnosticsPlugin,
    ));
    #[cfg(feature = "debug_physics")]
    app.add_plugins(bevy_rapier3d::render::RapierDebugRenderPlugin::default());
    if smoke {
        std::fs::create_dir_all("screenshots").expect("create screenshot directory");
        app.insert_resource(game::Persistence(false))
            .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
                std::time::Duration::from_secs_f64(1.0 / 60.0),
            ))
            .add_systems(Update, verification::smoke_test.after(ui::update_ui));
    }
    app
}
