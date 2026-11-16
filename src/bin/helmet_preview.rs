//! Isolated comparison of the production helmet shell and visor meshes.
use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    mesh::skinning::SkinnedMeshInverseBindposes,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use road_dash::models::{
    AuthoredModels, asset_directory,
    gear::{Helmet, Suit},
};

#[derive(Resource)]
struct Preview {
    suits: bool,
    capture: Option<String>,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let first = args.next();
    let suits = first.as_deref() == Some("--suits");
    let destination = if suits { args.next() } else { first };
    App::new()
        .insert_resource(Preview {
            suits,
            capture: destination,
        })
        .insert_resource(ClearColor(Color::srgb(0.19, 0.21, 0.24)))
        .insert_resource(GlobalAmbientLight {
            brightness: 1100.,
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
                        resolution: if suits { (1600, 1200) } else { (1200, 760) }.into(),
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
    preview: Res<Preview>,
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
                viewport_height: if preview.suits { 6.90 } else { 1.28 },
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0., 0., 2.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    for (position, illuminance) in [
        (Vec3::new(-2., 4., 3.), 17_000.),
        (Vec3::new(3., 1., 2.), 6_000.),
        (Vec3::new(1., 3., -3.), 11_000.),
    ] {
        commands.spawn((
            DirectionalLight {
                illuminance,
                ..default()
            },
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
    if preview.suits {
        let font: Handle<Font> = server.load("ui/fonts/Jost-700-Bold.ttf");
        let parts = models.suit_display(&mut meshes);
        for (index, suit) in Suit::ALL.into_iter().enumerate() {
            let column = index % 5;
            let row = index / 5;
            for (mesh, materials) in &parts {
                commands.spawn((
                    mesh.clone(),
                    materials[index].clone(),
                    Transform::from_xyz((column as f32 - 2.) * 1.81, 1.20 - row as f32 * 2.10, 0.)
                        .with_rotation(Quat::from_rotation_y(-0.36))
                        .with_scale(Vec3::splat(0.78)),
                ));
            }
            commands.spawn((
                Text::new(suit.name()),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(25.),
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: percent(19.7),
                    left: percent(0.7 + column as f32 * 19.7),
                    top: percent(33.2 + row as f32 * 30.43),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            ));
        }
        return;
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

fn capture(
    mut commands: Commands,
    mut frame: Local<u32>,
    mut exit: MessageWriter<AppExit>,
    preview: Res<Preview>,
) {
    let Some(path) = preview.capture.as_ref() else {
        return;
    };
    *frame += 1;
    if *frame == 120 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path.clone()));
    }
    if *frame == 150 {
        exit.write(AppExit::Success);
    }
}
