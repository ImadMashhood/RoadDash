//! Short native before/after view of the same route anchors and effect phases.
use super::{Kind, MapFxSprite, quality};
use crate::{
    game::{ChaseCamera, Phase, Race},
    runtime::AppScreen,
    track::Track,
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};

#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    captures: usize,
    last: Option<std::time::Instant>,
    steady_frames_ms: Vec<f64>,
}

fn captured(_: On<ScreenshotCaptured>, mut survey: ResMut<Survey>) {
    survey.captures += 1;
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    mut commands: Commands,
    mut survey: ResMut<Survey>,
    screen: Res<State<AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    mut cameras: Query<&mut Transform, With<ChaseCamera>>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    sprites: Query<(&MapFxSprite, &Visibility)>,
    metrics: Res<crate::diagnostics::Metrics>,
    clock: Res<crate::runtime::SimulationClock>,
    diagnostics: Option<Res<bevy::diagnostic::DiagnosticsStore>>,
    mut exit: MessageWriter<AppExit>,
) {
    if matches!(screen.get(), AppScreen::Boot | AppScreen::Loading) {
        return;
    }
    for (entity, parent) in &ui {
        if parent.is_none() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    survey.frame += 1;
    let frame = survey.frame;
    let now = std::time::Instant::now();
    if let Some(last) = survey.last.replace(now)
        && (151..=225).contains(&frame)
    {
        survey
            .steady_frames_ms
            .push((now - last).as_secs_f64() * 1000.);
    }
    let map = if track.is_hills() {
        "hills"
    } else if track.metropolis.is_some() {
        "metro"
    } else if track.city_surface.is_some() {
        "city"
    } else {
        "valley"
    };
    let stage = if track.is_hills() && frame >= 90 {
        1
    } else {
        0
    };
    let station = match map {
        "hills" if stage == 0 => 7900.,
        "hills" => 8000.,
        "metro" => 9000.,
        "city" => 1200.,
        _ => track.length * 0.28,
    };
    let (road, rotation) = track.sample_route(0, station, 0.);
    let side = rotation * Vec3::X;
    let sign = if map == "hills" || map == "city" {
        -1.
    } else {
        1.
    };
    let eye = road + side * sign * 31. + Vec3::Y * 3.8 - rotation * Vec3::Z * 11.;
    let target = road + side * sign * 19. + Vec3::Y * 1.1;
    if let Ok(mut camera) = cameras.single_mut() {
        *camera = Transform::from_translation(eye).looking_at(target, Vec3::Y);
    }
    race.phase = Phase::Paused;
    // The two pairs hold one camera/anchor and only change the race-time phase.
    let (name, elapsed) = if track.is_hills() {
        if frame < 45 {
            ("heat-small", 1.92)
        } else if frame < 90 {
            ("heat-large", 0.40)
        } else if frame < 105 {
            ("sand-small", 2.18)
        } else {
            ("sand-large", 0.61)
        }
    } else {
        let (small, large) = match map {
            "metro" => (1.0, 1.7),
            "city" => (4.5, 5.2),
            _ => (7.68, 0.61),
        };
        if frame < 45 {
            ("small", small)
        } else {
            ("large", large)
        }
    };
    race.elapsed = elapsed;
    let capture = match frame {
        30 | 60 => true,
        100 | 120 if track.is_hills() => true,
        _ => false,
    };
    if capture {
        let quality = match quality() {
            0 => "off",
            1 => "low",
            _ => "default",
        };
        let path = format!("screenshots/map-fx-{map}-{quality}-{name}.png");
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path))
            .observe(captured);
    }
    let expected = if track.is_hills() { 4 } else { 2 };
    if frame > 230 && survey.captures == expected {
        let active = sprites
            .iter()
            .filter(|(_, v)| **v == Visibility::Inherited)
            .count();
        let counts = [Kind::Sand, Kind::Heat, Kind::Steam, Kind::Pollen]
            .map(|kind| sprites.iter().filter(|(s, _)| s.kind == kind).count());
        let quality = match quality() {
            0 => "off",
            1 => "low",
            _ => "default",
        };
        let path = format!("screenshots/map-fx-{map}-{quality}.json");
        survey.steady_frames_ms.sort_by(f64::total_cmp);
        let samples = &survey.steady_frames_ms;
        let steady = serde_json::json!({
            "count":samples.len(),
            "median_ms":samples[samples.len()/2],
            "p95_ms":samples[samples.len()*95/100],
            "p99_ms":samples[samples.len()*99/100]
        });
        let render_diagnostics: std::collections::BTreeMap<_, _> = diagnostics
            .as_ref()
            .into_iter()
            .flat_map(|store| store.iter())
            .filter(|d| d.path().as_str().starts_with("render/"))
            .filter_map(|d| d.value().map(|value| (d.path().as_str().to_owned(), value)))
            .collect();
        std::fs::write(path, serde_json::to_vec_pretty(&serde_json::json!({
            "map":map,"quality":quality,"captures":survey.captures,"pool_counts":{"sand":counts[0],"heat":counts[1],"steam":counts[2],"pollen":counts[3]},"active_in_final_view":active,
            "steady_paused_view_wall_frames":steady,
            "latest_render_diagnostics":render_diagnostics,
            "metrics":metrics.evidence(&clock),
            "limits":"Paused fixed-view visual comparison; wall-frame sample is not GPU timing or gameplay performance. Effects are transparent-texture cues, not refraction."
        })).expect("serialize map FX report")).expect("write map FX report");
        exit.write(AppExit::Success);
    }
    assert!(frame < 340, "map FX screenshots timed out");
}
