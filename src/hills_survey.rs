//! Repeatable Hills camera and placed-road collision evidence.
use crate::{
    track::Track,
    world_gen::hills::{Census, HillsSolid},
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};
use bevy_rapier3d::prelude::*;

const VIEWS: &[(usize, f32, bool)] = &[
    (0, 150., false),
    (0, 1200., false),
    (0, 2500., false),
    (0, 4200., true),
    (0, 5500., true),
    (0, 6200., false),
    (1, 6200., false),
    (0, 7400., true),
    (0, 8200., false),
    (0, 9800., false),
    (0, 12300., true),
];

pub fn inspect(track: &Track) -> serde_json::Value {
    let elevations: Vec<_> = track.points.iter().map(|p| p.y).collect();
    let maximum_grade = track
        .points
        .windows(2)
        .map(|p| (p[1].y - p[0].y).abs() / (p[1] - p[0]).with_y(0.).length())
        .fold(0f32, f32::max);
    serde_json::json!({
        "id":track.definition.id, "name":track.definition.name,
        "length_m":track.length, "routes":2,
        "town_split_m":crate::track::hills::SPLIT, "town_rejoin_m":crate::track::hills::MERGE, "town_end_m":crate::track::hills::TOWN_END,
        "elevation_min_m":elevations.iter().copied().fold(f32::INFINITY,f32::min),
        "elevation_max_m":elevations.iter().copied().fold(f32::NEG_INFINITY,f32::max),
        "maximum_sampled_grade":maximum_grade,
        "checkpoints":track.definition.checkpoints.len(),
        "views":VIEWS.iter().map(|(route,s,aerial)|serde_json::json!({"route":route,"station_m":s,"aerial":aerial})).collect::<Vec<_>>()
    })
}

#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    screenshots: usize,
    solids: Option<usize>,
    views: Vec<serde_json::Value>,
}

fn captured(_: On<ScreenshotCaptured>, mut state: ResMut<Survey>) {
    state.screenshots += 1;
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    census: Res<Census>,
    mut race: ResMut<crate::game::Race>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    mut exit: MessageWriter<AppExit>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    metrics: Res<crate::diagnostics::Metrics>,
    clock: Res<crate::runtime::SimulationClock>,
    solids: Query<(Option<&Mesh3d>, Option<&Collider>, Option<&ViewVisibility>), With<HillsSolid>>,
    ground: Query<(), With<crate::world::Ground>>,
    physics: ReadRapierContext,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    for (entity, parent) in &ui {
        if parent.is_none() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    // Let Rapier build its broadphase/query state before pausing the camera survey.
    // Garage/Paused skip physics stepping, so a never-started race has no ray hits.
    race.phase = if state.frame < 3 {
        crate::game::Phase::Countdown
    } else {
        crate::game::Phase::Paused
    };
    state.frame += 1;
    let count = solids.iter().count();
    if let Some(expected) = state.solids {
        assert_eq!(
            count, expected,
            "camera movement must retain solid geometry"
        );
    } else {
        state.solids = Some(count);
    }
    assert!(count > 100);
    assert!(solids.iter().all(|(_, collider, _)| collider.is_some()));
    // Warm each view for 120 rendered frames after the normal asset readiness gate.
    let period = 120;
    let index = ((state.frame / period) as usize).min(VIEWS.len() - 1);
    let (route, s, aerial) = VIEWS[index];
    let (p, q) = track.sample_route(route, s, 0.);
    for mut camera in &mut cameras {
        let eye = if aerial {
            p + q * Vec3::new(130., 160., -150.)
        } else {
            p - q * Vec3::Z * 18. + Vec3::Y * 6.
        };
        let target = p + q * Vec3::Z * 35. + Vec3::Y * 3.;
        *camera = Transform::from_translation(eye).looking_at(target, Vec3::Y);
    }
    if state.frame % period == period - 10 && state.views.len() < VIEWS.len() {
        let visible = solids
            .iter()
            .filter(|(m, _, v)| m.is_some() && v.is_some_and(|v| v.get()))
            .count();
        assert!(
            visible > 0,
            "no placed Hills mesh visible in survey view {index}"
        );
        state.views.push(serde_json::json!({"view":index,"route":route,"station_m":s,"visible_solid_meshes":visible}));
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/hills-{index:02}.png")))
            .observe(captured);
    }
    if state.frame > period * VIEWS.len() as u32 + 20 && state.screenshots == VIEWS.len() {
        let context = physics.single().expect("Hills physics context");
        let is_ground = |e| ground.contains(e);
        let mut maximum_error = 0f32;
        let mut probes = 0;
        for route in 0..2 {
            for station in (0..=crate::track::hills::LENGTH as u32).step_by(50) {
                for lane in [-4., 0., 4.] {
                    let p = track.sample_route(route, station as f32, lane).0;
                    let (_, toi) = context
                        .cast_ray(
                            p + Vec3::Y * 3.,
                            Vec3::NEG_Y,
                            6.,
                            true,
                            QueryFilter::default().predicate(&is_ground),
                        )
                        .unwrap_or_else(|| panic!("missing Hills ground support: route={route} station={station} lane={lane} point={p:?}"));
                    let error = (3. - toi).abs();
                    maximum_error = maximum_error.max(error);
                    assert!(
                        error < 0.05,
                        "route {route} station {station} lane {lane}: road support error {error}"
                    );
                    probes += 1;
                }
            }
        }
        let report = serde_json::json!({"track":inspect(&track),"census":&*census,"views":state.views,"screenshots_completed":state.screenshots,"resident_solids":count,"road_support_probes":probes,"maximum_support_error_m":maximum_error,"metrics":metrics.evidence(&clock),"limits":"Camera survey with paused racers; not full-race performance, GPU timings, human art acceptance or hardware certification."});
        std::fs::write(
            "screenshots/hills-survey.json",
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .expect("write Hills survey");
        exit.write(AppExit::Success);
    }
    assert!(
        state.frame < period * VIEWS.len() as u32 + 600,
        "Hills screenshots timed out"
    );
}
