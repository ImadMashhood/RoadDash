//! Reproducible native fixture using the production registration/damage systems.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    props: Vec<Entity>,
    base: Vec3,
    peak: usize,
    production_props: usize,
    variant_roots: usize,
    variant_entities: usize,
    variant_meshes: usize,
    timings: [Vec<f64>; 3],
    last_frame: Option<std::time::Instant>,
    preparation_frames: u32,
    preparation_started: Option<std::time::Instant>,
}
#[derive(Component)]
pub struct Probe;
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub(crate) fn step(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    art: Res<crate::world::Art>,
    track: Res<crate::track::Track>,
    mut race: ResMut<Race>,
    props: Query<(&Breakable, &Transform, &RigidBody)>,
    probes: Query<Entity, With<Probe>>,
    mut reset: MessageWriter<Reset>,
    stats: Res<Stats>,
    mut exit: MessageWriter<AppExit>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    variants: (
        Query<&visuals::Visual>,
        Query<&Children>,
        Query<(), With<Mesh3d>>,
        Query<&Visibility>,
        Query<&crate::props::PropVisual>,
    ),
    forbidden: Query<
        (),
        (
            With<Breakable>,
            Or<(With<Traffic>, With<crate::environment::CrossTraffic>)>,
        ),
    >,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    if state.frame == 2 {
        let missing: Vec<_> = state
            .props
            .iter()
            .filter_map(|e| {
                let (p, _, _) = props.get(*e).ok()?;
                (p.profile.style == Style::Crush
                    && !variants.0.get(p.visual).is_ok_and(|v| v.prepared))
                .then(|| variants.4.get(p.visual).unwrap().id.clone())
            })
            .collect();
        if !missing.is_empty() {
            state.preparation_frames += 1;
            let started = *state
                .preparation_started
                .get_or_insert_with(std::time::Instant::now);
            assert!(
                started.elapsed().as_secs_f64() < 30. && state.preparation_frames < 3600,
                "damaged scene preparation timed out: {missing:?}"
            );
            state.last_frame = Some(std::time::Instant::now());
            return;
        }
    }
    state.frame += 1;
    let now = std::time::Instant::now();
    let wall_ms = state
        .last_frame
        .replace(now)
        .map_or(0., |last| now.duration_since(last).as_secs_f64() * 1000.);
    for (i, range) in [(0, 20..80), (1, 120..335), (2, 400..440)] {
        if range.contains(&state.frame) {
            state.timings[i].push(wall_ms);
        }
    }
    race.phase = Phase::Racing;
    for (e, parent) in &ui {
        if parent.is_none() {
            commands.entity(e).insert(Visibility::Hidden);
        }
    }
    if state.frame == 1 {
        state.production_props = props.iter().count();
        assert!(
            state.production_props > 0,
            "production world must be loaded"
        );
        let p = track.sample_route(0, -148., 0.).0 + Vec3::X * 34.;
        let ground = [
            Vec2::new(p.x - 24., p.z - 15.),
            Vec2::new(p.x + 24., p.z + 15.),
        ]
        .into_iter()
        .map(|p| track.terrain_height(p.x, p.y))
        .fold(p.y, f32::max)
            + 1.;
        state.base = Vec3::new(p.x, ground, p.z);
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(25., 0.1, 16.),
            crate::world::Ground,
            Transform::from_translation(state.base - Vec3::Y * 0.1),
        ));
        commands.spawn((
            Mesh3d(art.cube.clone()),
            MeshMaterial3d(art.dark.clone()),
            Transform::from_translation(state.base - Vec3::Y * 0.1)
                .with_scale(Vec3::new(50., 0.2, 32.)),
        ));
        let kit = art.props.as_ref().unwrap();
        let m = track.metropolis.as_ref().unwrap();
        for (i, kind) in Tuning::default().0.profiles.keys().enumerate() {
            let spec = m.street_props.iter().find(|p| &p.kind == kind).unwrap();
            let p = state.base + Vec3::X * (i as f32 * 3. - 18.);
            state.props.push(register(
                &mut commands,
                kit,
                kind,
                0,
                &format!("lab-{kind}"),
                Transform::from_translation(p),
                Vec3::from_array(spec.size),
                None,
            ));
        }
    }
    if [80, 340, 460].contains(&state.frame) {
        let label = if state.frame == 80 {
            "before"
        } else if state.frame == 340 {
            "damaged"
        } else {
            "reset"
        };
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/destruction-{label}.png")));
    }
    for (start, label) in [(84, "before"), (344, "damaged"), (440, "reset")] {
        if (start..start + 6).contains(&state.frame) {
            let kind = Tuning::default()
                .0
                .profiles
                .into_iter()
                .filter(|(_, p)| p.style == Style::Crush)
                .nth((state.frame - start) as usize)
                .unwrap()
                .0;
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!(
                    "screenshots/destruction-{label}-{kind}.png"
                )));
        }
    }
    if state.frame == 100 {
        for &e in &state.props {
            let (_, t, _) = props.get(e).unwrap();
            commands.spawn((
                Probe,
                RigidBody::Dynamic,
                Collider::ball(0.35),
                AdditionalMassProperties::Mass(180.),
                Velocity::linear(Vec3::Z * 20.),
                Ccd::enabled(),
                Transform::from_translation(t.translation + Vec3::new(0., 0.65, -3.)),
                ActiveEvents::COLLISION_EVENTS,
            ));
        }
    }
    if state.frame == 335 {
        let broken = state
            .props
            .iter()
            .filter(|e| props.get(**e).unwrap().0.state.broken)
            .count();
        assert_eq!(
            broken, 13,
            "every production prop kind must break in the real contact fixture"
        );
        assert!(stats.peak_dynamic <= 48);
        state.peak = stats.peak_dynamic;
        for &e in &state.props {
            let (p, _, _) = props.get(e).unwrap();
            if p.profile.style == Style::Crush {
                let v = variants.0.get(p.visual).unwrap();
                assert_eq!(*variants.3.get(p.visual).unwrap(), Visibility::Hidden);
                assert_eq!(
                    *variants.3.get(v.damaged.unwrap()).unwrap(),
                    Visibility::Inherited
                );
                assert!(v.prepared, "actual broken scene/materials ready");
            }
        }
        for v in &variants.0 {
            if let Some(root) = v.damaged {
                state.variant_roots += 1;
                state.variant_entities += 1 + variants.1.iter_descendants(root).count();
                state.variant_meshes += variants
                    .1
                    .iter_descendants(root)
                    .filter(|e| variants.2.contains(*e))
                    .count();
            }
        }
        assert_eq!(forbidden.iter().count(), 0);
        assert_eq!(
            props.iter().count(),
            state.production_props + state.props.len()
        );
    }
    if state.frame == 390 {
        for e in &probes {
            commands.entity(e).despawn();
        }
        reset.write(Reset);
    }
    if state.frame == 470 {
        for e in &state.props {
            let (p, t, body) = props.get(*e).unwrap();
            assert!(!p.state.broken);
            assert_eq!(*t, p.original);
            assert_eq!(*body, RigidBody::Fixed);
            let v = variants.0.get(p.visual).unwrap();
            assert_eq!(*variants.3.get(p.visual).unwrap(), Visibility::Inherited);
            if let Some(damaged) = v.damaged {
                assert_eq!(*variants.3.get(damaged).unwrap(), Visibility::Hidden);
            }
        }
        let metrics: Vec<_> = state.timings.iter().map(|samples| {
            let mut sorted = samples.clone(); sorted.sort_by(f64::total_cmp);
            serde_json::json!({"frames": sorted.len(), "p50_ms": sorted[sorted.len()/2], "p95_ms": sorted[(sorted.len()*95/100).min(sorted.len()-1)], "max_ms": sorted.last()})
        }).collect();
        std::fs::write("screenshots/destruction-results.json", serde_json::to_string_pretty(&serde_json::json!({
            "production_props": state.production_props, "tested_kinds": 13, "real_collision_breaks": 13,
            "restart_restored": true, "cars_damaged": 0, "peak_dynamic_props": state.peak,
            "variant_roots": state.variant_roots, "variant_entities": state.variant_entities,
            "variant_meshes": state.variant_meshes, "wall_frame_metrics_intact_damaged_reset": metrics,
            "preparation_wait_frames": state.preparation_frames, "frame": state.frame
        })).unwrap()).unwrap();
        exit.write(AppExit::Success);
    }
}
pub fn camera(
    state: Res<Survey>,
    props: Query<(&Transform, &Breakable), Without<Camera3d>>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    if state.frame == 0 {
        return;
    }
    let mut lo = state.base;
    let mut hi = state.base;
    for e in &state.props {
        if let Ok((t, _)) = props.get(*e) {
            lo = lo.min(t.translation);
            hi = hi.max(t.translation);
        }
    }
    for start in [84, 344, 440] {
        if (start..start + 6).contains(&state.frame) {
            let kind = Tuning::default()
                .0
                .profiles
                .into_iter()
                .filter(|(_, p)| p.style == Style::Crush)
                .nth((state.frame - start) as usize)
                .unwrap()
                .0;
            let index = Tuning::default()
                .0
                .profiles
                .keys()
                .position(|k| k == &kind)
                .unwrap();
            let (pose, p) = props.get(state.props[index]).unwrap();
            let height = p.size.y * if p.state.broken { 0.38 } else { 1. };
            let target = pose.translation + pose.rotation * Vec3::Y * height * 0.5;
            let distance = p.size.max_element().max(1.5) * 2.5;
            for mut camera in &mut cameras {
                *camera = Transform::from_translation(
                    target + Vec3::new(distance * 0.8, distance * 0.55, -distance),
                )
                .looking_at(target, Vec3::Y);
            }
            return;
        }
    }
    let center = (lo + hi) * 0.5;
    let width = (hi.x - lo.x).max(38.);
    for mut t in &mut cameras {
        *t = Transform::from_translation(center + Vec3::new(8., width * 0.65, -width * 0.85))
            .looking_at(center + Vec3::Y, Vec3::Y);
    }
}
