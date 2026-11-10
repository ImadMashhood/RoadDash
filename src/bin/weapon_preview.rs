//! Native original weapon GLB inspector. No race, physics or save systems are installed.
use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
use rr_content::weapon_models::Catalog;
use std::path::PathBuf;

#[derive(Resource)]
struct Viewer {
    catalog: Catalog,
    index: usize,
    lod: usize,
    yaw: f32,
    distance: f32,
    changed: bool,
    capture: Option<PathBuf>,
    frames: u32,
    finishing_capture: bool,
}
#[derive(Component)]
struct Model;
#[derive(Component)]
struct Label;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let value = |flag| {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i + 1))
    };
    if args.iter().any(|a| a == "--equipped") {
        road_dash::animation::preview::build_weapon_review(value("--capture").map(PathBuf::from))
            .run();
        return;
    }
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");
    let catalog = Catalog::builtin();
    let index = value("--id")
        .map(|id| {
            catalog
                .records
                .iter()
                .position(|r| &r.id == id || &r.name == id)
                .expect("unknown weapon model ID")
        })
        .unwrap_or(0);
    let lod = value("--lod")
        .map(|v| v.parse::<usize>().expect("LOD must be 0, 1 or 2"))
        .unwrap_or(0);
    assert!(lod < 3, "LOD must be 0, 1 or 2");
    App::new()
        .add_plugins(road_dash::world_gen::city::visuals::BuildingSceneTypesPlugin)
        .insert_resource(Viewer {
            catalog,
            index,
            lod,
            yaw: -0.55,
            distance: 1.0,
            changed: true,
            capture: value("--capture").map(PathBuf::from),
            frames: 0,
            finishing_capture: false,
        })
        .insert_resource(ClearColor(Color::srgb(0.065, 0.075, 0.09)))
        .insert_resource(AmbientLight {
            brightness: 700.0,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: root.to_string_lossy().into_owned(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "RoadDash — original 3D weapons | arrows: asset, 1–3: LOD, Q/E: orbit, +/-: zoom".into(),
                        resolution: (1280, 900).into(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, (input, show, camera, capture).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Tonemapping::AcesFitted));
    commands.spawn((
        DirectionalLight {
            illuminance: 14000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(5.0, 8.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Text::default(),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            left: px(15),
            top: px(15),
            ..default()
        },
        Label,
    ));
}

fn input(keys: Res<ButtonInput<KeyCode>>, mut v: ResMut<Viewer>, time: Res<Time>) {
    let len = v.catalog.records.len();
    if keys.just_pressed(KeyCode::ArrowRight) {
        v.index = (v.index + 1) % len;
        v.changed = true;
    }
    if keys.just_pressed(KeyCode::ArrowLeft) {
        v.index = (v.index + len - 1) % len;
        v.changed = true;
    }
    for (key, lod) in [
        (KeyCode::Digit1, 0),
        (KeyCode::Digit2, 1),
        (KeyCode::Digit3, 2),
    ] {
        if keys.just_pressed(key) {
            v.lod = lod;
            v.changed = true;
        }
    }
    let dt = time.delta_secs();
    if keys.pressed(KeyCode::KeyQ) {
        v.yaw += dt;
    }
    if keys.pressed(KeyCode::KeyE) {
        v.yaw -= dt;
    }
    if keys.pressed(KeyCode::Equal) {
        v.distance = (v.distance - dt).max(0.4);
    }
    if keys.pressed(KeyCode::Minus) {
        v.distance = (v.distance + dt).min(4.0);
    }
}

fn show(
    mut commands: Commands,
    mut v: ResMut<Viewer>,
    server: Res<AssetServer>,
    models: Query<Entity, With<Model>>,
    mut labels: Query<&mut Text, With<Label>>,
) {
    if !v.changed {
        return;
    }
    for entity in &models {
        commands.entity(entity).despawn();
    }
    let prop = &v.catalog.records[v.index];
    let scene: Handle<Scene> = server.load(format!("{}#Scene0", prop.lods[v.lod].file));
    commands.spawn((SceneRoot(scene), Model));
    for mut label in &mut labels {
        label.0 = format!(
            "{} | {}\nLOD {}: {} triangles | {}/{}\nPresentation models · original club ability unchanged · grip origin: zero, axis: +Y\nArrows: asset · 1/2/3: LOD · Q/E: orbit · +/-: zoom",
            prop.id,
            prop.name,
            v.lod,
            prop.lods[v.lod].triangles,
            v.index + 1,
            v.catalog.records.len()
        );
    }
    v.changed = false;
    v.frames = 0;
}

fn camera(v: Res<Viewer>, mut query: Query<&mut Transform, With<Camera3d>>) {
    let p = &v.catalog.records[v.index];
    let bounds = p.lods[v.lod].bounds_y_up;
    let min = Vec3::from_array(bounds[0]);
    let max = Vec3::from_array(bounds[1]);
    let target = (min + max) * 0.5;
    let size = (max - min).max_element().max(0.5);
    for mut transform in &mut query {
        *transform = Transform::from_translation(
            target + Vec3::new(v.yaw.sin(), 0.4, v.yaw.cos()) * size * 1.7 * v.distance,
        )
        .looking_at(target, Vec3::Y);
    }
}

fn capture(
    mut commands: Commands,
    mut v: ResMut<Viewer>,
    server: Res<AssetServer>,
    models: Query<&SceneRoot, With<Model>>,
    mut exit: MessageWriter<AppExit>,
) {
    v.frames += 1;
    if v.capture.is_none() {
        if v.finishing_capture && v.frames > 20 {
            exit.write(AppExit::Success);
        }
        return;
    }
    if v.frames > 120
        && models
            .iter()
            .all(|s| server.is_loaded_with_dependencies(s.id()))
    {
        let path = v.capture.take().expect("capture path");
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("capture folder");
        }
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
        v.frames = 0;
        v.finishing_capture = true;
    } else if v.frames > 180 {
        panic!("Weapon scene did not finish loading for capture");
    }
}
