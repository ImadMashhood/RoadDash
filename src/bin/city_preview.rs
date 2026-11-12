//! Isolated architectural design viewer: no race state, Rapier or save mutations.
use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[path = "city_preview/models.rs"]
mod models;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
struct Scene {
    length_m: f32,
    chunks: Vec<Chunk>,
    lots: Vec<models::Lot>,
}
type Chunk = rr_content::city::CityChunk;
#[cfg(test)]
use road_dash::world_gen::city::geometry::chunk_mesh;
#[derive(Resource)]
struct Options {
    scene: PathBuf,
    captures: Option<PathBuf>,
    survey: bool,
}
#[derive(Resource, Default)]
struct CaptureFrame {
    frame: u32,
    last: Option<std::time::Instant>,
    samples_ms: Vec<f64>,
}
#[derive(Component)]
struct PreviewCamera;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let argument = |flag| {
        args.iter()
            .position(|x| x == flag)
            .and_then(|i| args.get(i + 1))
            .map(PathBuf::from)
    };
    let captures = argument("--capture");
    if let Some(path) = &captures {
        std::fs::create_dir_all(path).expect("capture directory");
    }
    let mut app = App::new();
    road_dash::world_gen::city::register_assets(&mut app);
    app.add_plugins(models::BuildingSceneTypesPlugin)
        .insert_resource(Options {
            scene: argument("--scene").unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("art/buildings/city_preview/generated/scene.json")),
            captures,
            survey: args.iter().any(|a| a == "--catalog-survey"),
        })
        .insert_resource(ClearColor(Color::srgb(0.43, 0.53, 0.62)))
        .insert_resource(GlobalAmbientLight { color: Color::srgb(0.83,0.88,1.0), brightness: 800.0, ..default() })
        .init_resource::<road_dash::material_detail::Queue>()
        .init_resource::<road_dash::tangent_detail::Queue>()
        .init_resource::<CaptureFrame>()
        .add_plugins(DefaultPlugins.set(AssetPlugin { file_path: format!("{}/art/buildings/mesh_assets/models", env!("CARGO_MANIFEST_DIR")), ..default() }).set(WindowPlugin {
            primary_window: Some(Window {
                title: "RoadDash — 1 km city design study | WASD move, R/F height, right mouse look, 1–5 views".into(),
                resolution: (1600, 1000).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }), ..default()
        }))
        .add_systems(Update,(road_dash::material_detail::update,road_dash::tangent_detail::update).chain().before(models::update_lod))
        .add_systems(Startup, setup)
        .add_systems(Update, (controls, models::update_lod, models::fade, capture).chain())
        .add_systems(PostUpdate,road_dash::world_gen::architecture::render::synchronize_fades.after(bevy::transform::TransformSystems::Propagate).before(bevy::camera::visibility::VisibilitySystems::CheckVisibility))
        .run();
}

fn pose(index: usize) -> Transform {
    let (eye, target) = match index {
        0 => (Vec3::new(250.0, 310.0, -240.0), Vec3::new(0.0, 0.0, 470.0)),
        1 => (Vec3::new(1.75, 2.7, 220.0), Vec3::new(1.75, 4.0, 300.0)),
        2 => (Vec3::new(-2.5, 7.0, 298.0), Vec3::new(16.0, 7.0, 325.0)),
        3 => (Vec3::new(102.0, 72.0, 382.0), Vec3::new(0.0, 0.0, 470.0)),
        _ => (Vec3::new(1973.0, 8.0, 9.0), Vec3::new(2000.0, 5.2, 13.25)),
    };
    Transform::from_translation(eye).looking_at(target, Vec3::Y)
}

fn setup(
    mut commands: Commands,
    options: Res<Options>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    server: Res<AssetServer>,
) {
    let scene: Scene = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(&options.scene).expect("Run python3 tools/build_city_preview.py first"),
    ))
    .expect("valid city scene");
    models::spawn(&mut commands, &server, &scene.lots, true);
    let count: usize = scene.chunks.iter().map(|c| c.boxes.len()).sum();
    road_dash::world_gen::city::geometry::spawn(
        &mut commands,
        &mut meshes,
        &mut materials,
        Some(&server),
        &scene.chunks,
    );
    println!(
        "City preview: {} m, {count} road/furniture boxes; actual GLB building models, no facade texture cards.",
        scene.length_m
    );
    commands.spawn((Text::new("ACTUAL 3D GLB MODELS | 100 facade designs / 146 catalog assets / 3 LODs | 5: gallery, PgUp/PgDn: page | WASD + right mouse to inspect"),
        TextFont { font_size: FontSize::Px(15.0), ..default() }, TextColor(Color::WHITE),
        Node { position_type: PositionType::Absolute, left: px(12), top: px(10), ..default() }));
    commands.spawn((
        DirectionalLight {
            illuminance: 16000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        bevy::light::CascadeShadowConfigBuilder {
            maximum_distance: 160.0,
            ..default()
        }
        .build(),
        Transform::from_xyz(80.0, 130.0, 70.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        PreviewCamera,
        pose(1),
        Tonemapping::Reinhard,
        DistanceFog {
            color: Color::srgb(0.43, 0.53, 0.62),
            falloff: FogFalloff::Linear {
                start: 250.0,
                end: 1200.0,
            },
            ..default()
        },
    ));
}

#[allow(
    clippy::too_many_arguments,
    reason = "Bevy system parameters are independently borrowed resources"
)]
fn controls(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    motion: Res<AccumulatedMouseMotion>,
    options: Res<Options>,
    mut cameras: Query<&mut Transform, With<PreviewCamera>>,
    mut exit: MessageWriter<AppExit>,
    mut page: Local<usize>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
    if options.captures.is_some() {
        return;
    }
    let Ok(mut t) = cameras.single_mut() else {
        return;
    };
    for (i, key) in [
        KeyCode::Digit1,
        KeyCode::Digit2,
        KeyCode::Digit3,
        KeyCode::Digit4,
        KeyCode::Digit5,
    ]
    .into_iter()
    .enumerate()
    {
        if keys.just_pressed(key) {
            *t = pose(i);
        }
    }
    if keys.just_pressed(KeyCode::PageDown) {
        *page = (*page + 1) % 25;
        *t = gallery_pose(*page);
    }
    if keys.just_pressed(KeyCode::PageUp) {
        *page = (*page + 24) % 25;
        *t = gallery_pose(*page);
    }
    let mut movement = Vec3::ZERO;
    let forward = Vec3::new(t.forward().x, 0.0, t.forward().z).normalize_or_zero();
    let right = Vec3::new(t.right().x, 0.0, t.right().z).normalize_or_zero();
    for (key, direction) in [
        (KeyCode::KeyW, forward),
        (KeyCode::KeyS, -forward),
        (KeyCode::KeyD, right),
        (KeyCode::KeyA, -right),
        (KeyCode::KeyR, Vec3::Y),
        (KeyCode::KeyF, Vec3::NEG_Y),
    ] {
        if keys.pressed(key) {
            movement += direction;
        }
    }
    t.translation += movement.normalize_or_zero()
        * time.delta_secs()
        * if keys.pressed(KeyCode::ShiftLeft) {
            80.0
        } else {
            15.0
        };
    t.translation.y = t.translation.y.max(0.5);
    if mouse.pressed(MouseButton::Right) {
        let (yaw, pitch, _) = t.rotation.to_euler(EulerRot::YXZ);
        t.rotation = Quat::from_euler(
            EulerRot::YXZ,
            yaw - motion.delta.x * 0.003,
            (pitch - motion.delta.y * 0.003).clamp(-1.5, 1.5),
            0.0,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn gallery_pose(page: usize) -> Transform {
    let offset = page as f32 * 50.0;
    Transform::from_xyz(1973.0 + offset, 8.0, 9.0)
        .looking_at(Vec3::new(2000.0 + offset, 5.2, 13.25), Vec3::Y)
}

#[allow(
    clippy::too_many_arguments,
    reason = "Bounded Bevy capture system with independently borrowed resources"
)]
fn capture(
    mut commands: Commands,
    options: Res<Options>,
    mut frame: ResMut<CaptureFrame>,
    windows: Query<&Window>,
    server: Res<AssetServer>,
    catalog: Res<models::ModelCatalog>,
    mut cameras: Query<&mut Transform, With<PreviewCamera>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(path) = &options.captures else {
        return;
    };
    if !catalog.ready(&server) {
        return;
    }
    let now = std::time::Instant::now();
    if let Some(last) = frame.last.replace(now)
        && frame.frame > 60
    {
        frame
            .samples_ms
            .push(now.duration_since(last).as_secs_f64() * 1000.0);
    }
    frame.frame += 1;
    let count = if options.survey { 25 } else { 5 };
    let index = ((frame.frame.saturating_sub(60)) / 90).min(count - 1) as usize;
    if let Ok(mut t) = cameras.single_mut() {
        *t = if options.survey {
            gallery_pose(index)
        } else {
            pose(index)
        };
    }
    if frame.frame >= 120 && (frame.frame - 120).is_multiple_of(90) && frame.frame < 60 + count * 90
    {
        let names = [
            "01-avenue-overview.png",
            "02-street-height.png",
            "03-junction.png",
            "04-block-grid.png",
            "05-actual-glb-models.png",
        ];
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path.join(if options.survey {
                format!("catalog-{:02}.png", index + 1)
            } else {
                names[index].to_string()
            })));
    }
    if frame.frame == 90 + count * 90 {
        frame.samples_ms.sort_by(f64::total_cmp);
        let samples = &frame.samples_ms;
        let report = serde_json::json!({"resolution":windows.single().ok().map(|w|[w.resolution.physical_width(),w.resolution.physical_height()]),"frames":frame.frame,"sample_count":samples.len(),"median_wall_frame_ms":samples[samples.len()/2],"p95_wall_frame_ms":samples[(samples.len()*95/100).min(samples.len()-1)],"limitation":"Bounded preview wall-frame sample with vsync and screenshot IO; not GPU timing, racing or production performance acceptance","physics":"None; isolated visual study","actual_glb_building_instances":catalog.instances,"mesh_designs":catalog.facade_designs,"catalog_assets":catalog.scenes.len(),"available_glb_lods":catalog.scenes.len()*3,"requested_glb_lods":catalog.requested_lods(),"facade_cards_loaded":0});
        std::fs::write(
            path.join("native-report.json"),
            serde_json::to_string_pretty(&report).expect("report serialization"),
        )
        .expect("write report");
        exit.write(AppExit::Success);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::mesh::{Indices, VertexAttributeValues};

    #[test]
    fn box_faces_have_outward_winding_and_exact_bounds() {
        let chunk = Chunk {
            name: "test".into(),
            material: None,
            boxes: vec![("wall".into(), [10., 3., 20.], [4., 6., 8.], [128, 64, 32])],
        };
        let mesh = chunk_mesh(&chunk);
        let Some(VertexAttributeValues::Float32x3(p)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("positions")
        };
        let Some(VertexAttributeValues::Float32x3(n)) = mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
        else {
            panic!("normals")
        };
        let Some(Indices::U32(indices)) = mesh.indices() else {
            panic!("indices")
        };
        for face in indices.chunks_exact(3) {
            let a = Vec3::from_array(p[face[0] as usize]);
            let b = Vec3::from_array(p[face[1] as usize]);
            let c = Vec3::from_array(p[face[2] as usize]);
            assert!(
                (b - a)
                    .cross(c - a)
                    .dot(Vec3::from_array(n[face[0] as usize]))
                    > 0.0
            );
        }
        let min = p.iter().fold(Vec3::splat(f32::INFINITY), |a, v| {
            a.min(Vec3::from_array(*v))
        });
        let max = p.iter().fold(Vec3::splat(f32::NEG_INFINITY), |a, v| {
            a.max(Vec3::from_array(*v))
        });
        assert_eq!(min, Vec3::new(8., 0., 16.));
        assert_eq!(max, Vec3::new(12., 6., 24.));
    }
}
