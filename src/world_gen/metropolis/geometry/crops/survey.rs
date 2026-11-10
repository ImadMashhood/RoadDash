//! Controlled native contact fixture: production shapes/stamping, scripted trajectories.
//! It does not claim to be a free-running physics crash or a GPU benchmark.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
struct Survey {
    frame: u32,
    warmup: u32,
    base: Vec3,
    probe: Option<Entity>,
    saved: usize,
    crash_cells: usize,
    stationary_bytes: u64,
    pause_time: f32,
    last_frame: Option<std::time::Instant>,
    quiet_ms: Vec<f64>,
    contact_ms: Vec<f64>,
}
#[derive(Component)]
struct Probe;
pub(super) fn install(app: &mut App) {
    app.init_resource::<Survey>()
        .insert_resource(crate::game::Persistence(false))
        .insert_resource(bevy::time::TimeUpdateStrategy::ManualDuration(
            std::time::Duration::from_secs_f64(1. / 60.),
        ))
        .insert_resource(bevy::winit::WinitSettings::continuous())
        .add_systems(Update, step.after(upload).after(crate::ui::update_ui))
        .add_systems(
            PostUpdate,
            camera
                .after(crate::game::camera)
                .before(bevy::transform::TransformSystems::Propagate),
        );
}
fn damaged(crops: &Crops) -> usize {
    crops
        .field
        .tiles
        .values()
        .flat_map(|t| t.cells.iter())
        .filter(|v| v[2] > 0.)
        .count()
}
#[allow(clippy::too_many_arguments)]
fn step(
    mut commands: Commands,
    mut survey: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    crops: Res<Crops>,
    mut race: ResMut<Race>,
    mut probes: Query<&mut Transform, With<Probe>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut reset: MessageWriter<crate::destruction::Reset>,
    mut exit: MessageWriter<AppExit>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    batches: Query<(&CropBatch, &ViewVisibility)>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    // Let native pipelines/mesh uploads settle at the fixed crop camera before
    // taking the intact reference or measuring either render mode.
    if survey.frame == 1 && survey.warmup < 240 {
        survey.warmup += 1;
        return;
    }
    survey.frame += 1;
    let now = std::time::Instant::now();
    let ms = survey
        .last_frame
        .replace(now)
        .map_or(0., |last| now.duration_since(last).as_secs_f64() * 1000.);
    if (20..60).contains(&survey.frame) {
        survey.quiet_ms.push(ms);
    }
    if (80..185).contains(&survey.frame) {
        survey.contact_ms.push(ms);
    }

    race.phase = if (70..190).contains(&survey.frame) {
        crate::game::Phase::Racing
    } else {
        crate::game::Phase::Paused
    };
    for (e, parent) in &ui {
        if parent.is_none() {
            commands.entity(e).insert(Visibility::Hidden);
        }
    }
    if survey.frame == 1 {
        std::fs::create_dir_all("screenshots").unwrap();
        let (key, _) = crops
            .field
            .tiles
            .iter()
            .max_by_key(|(_, t)| t.planted.iter().filter(|p| **p).count())
            .expect("wheat crops");
        let x = key.0 as f32 * SIDE as f32 + 8.;
        let z = key.1 as f32 * SIDE as f32 + 8.;
        survey.base = Vec3::new(x, track.terrain_height(x, z), z);
    }
    if survey.frame == 70 {
        survey.probe = Some(
            commands
                .spawn((
                    Probe,
                    OwnedBody {
                        owner: Entity::PLACEHOLDER,
                        kind: crate::recovery::BodyKind::Wreck,
                    },
                    Collider::cuboid(0.5, 0.35, 1.1),
                    Transform::from_translation(survey.base + Vec3::new(-5., 3., 0.)),
                    Mesh3d(meshes.add(Cuboid::new(1., 0.7, 2.2))),
                    MeshMaterial3d(materials.add(Color::srgb(0.2, 0.13, 0.08))),
                ))
                .id(),
        );
    }
    for mut t in &mut probes {
        let frame = survey.frame as f32;
        let x = if frame < 110. {
            -5. + (frame - 70.) * 0.12
        } else {
            (frame.min(160.) - 110.) * 0.07
        };
        let y = if frame < 88. {
            3. - (frame - 70.) * 0.15
        } else if frame < 110. {
            0.4
        } else {
            0.85
        };
        let p = survey.base + Vec3::new(x, 0., if frame < 110. { 0. } else { 2. });
        t.translation = p.with_y(track.terrain_height(p.x, p.z) + y);
    }
    if survey.frame == 110 {
        survey.crash_cells = damaged(&crops);
        assert!(survey.crash_cells > 0);
        let e = survey.probe.unwrap();
        commands.entity(e).insert((
            OwnedBody {
                owner: Entity::PLACEHOLDER,
                kind: crate::recovery::BodyKind::Runner,
            },
            Collider::capsule_y(0.5, 0.3),
            Mesh3d(meshes.add(Capsule3d::new(0.3, 1.))),
        ));
    }
    if survey.frame == 165 {
        survey.stationary_bytes = crops.uploaded_tile_bytes;
    }
    if survey.frame == 180 {
        assert_eq!(
            crops.uploaded_tile_bytes, survey.stationary_bytes,
            "stationary marks upload no tile data"
        );
    }
    if survey.frame == 185 {
        commands.entity(survey.probe.take().unwrap()).despawn();
    }
    if survey.frame == 195 {
        survey.saved = damaged(&crops);
        survey.pause_time = crops.time;
        assert!(
            survey.saved > survey.crash_cells,
            "walking must extend the crash mark"
        );
    }
    if survey.frame == 235 {
        assert_eq!(damaged(&crops), survey.saved);
        assert_eq!(crops.time, survey.pause_time);
        reset.write(crate::destruction::Reset);
    }
    for (frame, label) in [
        (60, "before"),
        (95, "bending"),
        (195, "trail"),
        (230, "persistent"),
        (255, "reset"),
    ] {
        if survey.frame == frame {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!("screenshots/crops-{label}.png")));
        }
    }
    if survey.frame == 265 {
        assert_eq!(damaged(&crops), 0);
        std::fs::write("screenshots/crops-survey.json",serde_json::to_string_pretty(&serde_json::json!({"tiles":crops.field.tiles.len(),"visible_crop_batches":batches.iter().filter(|(_,v)|v.get()).count(),"visible_crop_triangles":batches.iter().filter(|(_,v)|v.get()).map(|(b,_)|b.triangles).sum::<usize>(),"deformation_bytes":crops.field.tiles.len()*CELLS*16,"uploaded_tile_bytes_including_reset":crops.uploaded_tile_bytes,"dirty_tile_uploads_including_reset":crops.uploaded_tile_bytes/(CELLS*16) as u64,"stationary_contact_tile_bytes_per_frame":0,"quiet_wall_frame_ms":summary(&survey.quiet_ms),"contact_wall_frame_ms":summary(&survey.contact_ms),"crash_cells":survey.crash_cells,"damaged_cells_before_reset":survey.saved,"paused_clock_and_persistence_verified":true,"reset_verified":true,"fixture":"scripted physical wreck/capsule contact shapes; no free-running crash/GPU-timing claim"})).unwrap()).unwrap();
        exit.write(AppExit::Success);
    }
}
fn camera(survey: Res<Survey>, mut cameras: Query<&mut Transform, With<Camera3d>>) {
    if survey.frame == 0 {
        return;
    }
    for mut camera in &mut cameras {
        *camera = Transform::from_translation(survey.base + Vec3::new(-9., 7., 13.))
            .looking_at(survey.base + Vec3::Y * 0.5, Vec3::Y);
    }
}

fn summary(values: &[f64]) -> serde_json::Value {
    let mut sorted = values.to_vec();
    sorted.sort_by(f64::total_cmp);
    serde_json::json!({"samples":sorted.len(),"mean":sorted.iter().sum::<f64>()/sorted.len().max(1) as f64,"p95":sorted.get((sorted.len() as f64*0.95) as usize).copied().unwrap_or(0.),"limits":"whole application wall frames, synthetic contact and racing actor activity differ; not GPU timestamps or isolated algorithm A/B"})
}
