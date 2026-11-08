//! Native GPU comparison: no-range, degenerate range and safe finite range.
use bevy::{
    camera::visibility::VisibilityRange,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[derive(Resource, Default)]
struct Frames(u32);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (600, 300).into(),
                title: "RoadDash visibility range GPU probe".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Frames>()
        .add_systems(Startup, setup)
        .add_systems(Update, capture)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        bevy::core_pipeline::tonemapping::Tonemapping::None,
        Transform::from_xyz(0., 0., 13.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let cube = meshes.add(Cuboid::new(2.0, 2.0, 2.0));
    for (index, color) in [
        Color::srgb(1., 0., 0.),
        Color::srgb(0., 1., 0.),
        Color::srgb(0., 0., 1.),
    ]
    .into_iter()
    .enumerate()
    {
        let mut entity = commands.spawn((
            Mesh3d(cube.clone()),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: color,
                unlit: true,
                ..default()
            })),
            Transform::from_xyz((index as f32 - 1.) * 3.4, 0., 0.),
        ));
        if index == 1 {
            entity.insert(VisibilityRange {
                end_margin: 100.0..101.0,
                ..default()
            });
        }
        if index == 2 {
            entity.insert(VisibilityRange {
                start_margin: -1.0..0.0,
                end_margin: 100.0..101.0,
                use_aabb: false,
            });
        }
    }
}
fn capture(mut frames: ResMut<Frames>, mut commands: Commands, mut exit: MessageWriter<AppExit>) {
    frames.0 += 1;
    if frames.0 == 60 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("/tmp/roaddash-visibility-probe.png"));
    }
    if frames.0 == 100 {
        exit.write(AppExit::Success);
    }
}
