mod game;
mod track;
mod ui;
mod verification;
mod world;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ROAD / DASH — Pacific Run".into(),
            resolution: (1440, 900).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .init_resource::<track::Track>()
    .init_resource::<game::Race>()
    .init_resource::<game::Profile>()
    .init_resource::<game::Persistence>()
    .init_resource::<ui::MenuAction>()
    .insert_resource(TimestepMode::Variable {
        max_dt: 0.05,
        time_scale: 1.0,
        substeps: 3,
    })
    .add_systems(
        Startup,
        (world::setup_world, game::setup_race, ui::setup_ui).chain(),
    )
    .add_systems(
        Update,
        (
            ui::buttons,
            game::input,
            game::drive,
            game::camera,
            world::animate_riders,
            game::camera_occlusion,
            game::debris_cleanup,
            ui::update_ui,
        )
            .chain(),
    );
    if std::env::args().any(|a| a == "--smoke-test") {
        std::fs::create_dir_all("screenshots").expect("create screenshot directory");
        app.insert_resource(game::Persistence(false));
        app.insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1.0 / 60.0),
        ));
        app.add_systems(Update, verification::smoke_test.after(ui::update_ui));
    }
    app.run();
}
