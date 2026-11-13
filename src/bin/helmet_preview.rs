//! Isolated comparison of the production helmet shell and visor meshes.
use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    mesh::skinning::SkinnedMeshInverseBindposes,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use road_dash::models::{AuthoredModels, asset_directory, gear::Helmet};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.19, 0.21, 0.24)))
        .insert_resource(GlobalAmbientLight {
            brightness: 1800.,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: asset_directory(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RoadDash Helmet Comparison".into(),
                        resolution: (1200, 760).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, capture)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut binds: ResMut<Assets<SkinnedMeshInverseBindposes>>,
    mut images: ResMut<Assets<Image>>,
    server: Res<AssetServer>,
) {
    let models = AuthoredModels::load(
        &mut meshes,
        &mut materials,
        &mut binds,
        &mut images,
        Some(&server),
    );
    commands.spawn((
        Camera3d::default(),
        Tonemapping::AcesFitted,
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: bevy::camera::ScalingMode::FixedVertical {
                viewport_height: 1.28,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 2.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    for (position, illuminance) in [
        (Vec3::new(-2., 4., 3.), 27_000.),
        (Vec3::new(3., 1., 2.), 9_000.),
        (Vec3::new(1., 3., -3.), 17_000.),
    ] {
        commands.spawn((
            DirectionalLight {
                illuminance,
                ..default()
            },
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
    for (index, helmet) in Helmet::ALL.into_iter().enumerate() {
        let x = (index as f32 - 1.) * 0.63;
        for (y, yaw) in [(0.20, -0.48), (-0.32, -1.56)] {
            for (mesh, material) in models.helmet_display(&mut meshes, helmet) {
                commands.spawn((
                    mesh,
                    material,
                    Transform::from_xyz(x, y, 0.).with_rotation(Quat::from_euler(
                        EulerRot::YXZ,
                        yaw,
                        0.10,
                        0.,
                    )),
                ));
            }
        }
        commands.spawn((
            Text::new(helmet.name()),
            TextFont {
                font_size: FontSize::Px(27.),
                ..default()
            },
            Node {
                position_type: PositionType::Absolute,
                left: percent(15. + index as f32 * 31.2),
                top: percent(8.),
                ..default()
            },
        ));
    }
}

fn capture(mut commands: Commands, mut frame: Local<u32>, mut exit: MessageWriter<AppExit>) {
    let Some(path) = std::env::args().nth(1) else {
        return;
    };
    *frame += 1;
    if *frame == 120 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
    if *frame == 150 {
        exit.write(AppExit::Success);
    }
}
