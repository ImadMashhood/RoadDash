//! Native 1080p survey of all fifteen construction sections and a warm city sample.
use super::{buildings::GeneratedBuilding, layout::WorldLayout};
use crate::{
    diagnostics::Metrics,
    game::{ChaseCamera, Phase, Race},
    runtime::SimulationClock,
    track::Track,
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    benchmark_passed: bool,
}
#[allow(clippy::too_many_arguments)]
pub fn run(
    mut commands: Commands,
    mut survey: ResMut<Survey>,
    track: Res<Track>,
    plan: Res<super::WorldPlan>,
    layout: Res<WorldLayout>,
    mut camera: Query<&mut Transform, (With<ChaseCamera>, Without<crate::game::Rider>)>,
    mut riders: Query<(
        &mut crate::game::Rider,
        &mut crate::vehicle::BikeState,
        &mut Transform,
        &mut crate::runtime::PoseHistory,
    )>,
    mut race: ResMut<Race>,
    metrics: Res<Metrics>,
    clock: Res<SimulationClock>,
    buildings: Query<&GeneratedBuilding>,
    sinks: Query<&AudioSink>,
    mut exit: MessageWriter<AppExit>,
    windows: Query<&Window>,
    lods: Query<(&super::buildings::BuildingLod, &ViewVisibility)>,
) {
    survey.frame += 1;
    let frame = survey.frame;
    if frame == 60 {
        race.phase = Phase::Racing;
        assert!(sinks.iter().count() >= 4);
    }
    let stage = ((frame.saturating_sub(300)) / 90).min(15) as usize;
    let d = if stage < 15 {
        let s = &plan.definition.sections[stage];
        (s.start_m + s.end_m) * 0.5
    } else {
        4270.0
    };
    let (p, q) = super::layout::horizontal_frame(&track, d);
    if let Ok(mut t) = camera.single_mut() {
        *t = Transform::from_translation(
            p + q * Vec3::new(
                if stage == 15 { 35.0 } else { 55.0 },
                if stage == 15 { 24.0 } else { 38.0 },
                -75.0,
            ),
        )
        .looking_at(p + q * Vec3::new(0.0, 4.0, 45.0), Vec3::Y);
    }
    if frame >= 300 && stage < 15 && (frame - 300) % 90 == 75 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/world-S{:02}.png",
                stage + 1
            )));
    }
    if frame == 2120 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk("screenshots/world-city-benchmark.png"));
    }
    if (2200..2280).contains(&frame) {
        let (road, facing) = track.sample(4160.0, 0.0);
        if let Ok(mut t) = camera.single_mut() {
            *t = Transform::from_translation(road + Vec3::Y * 3.2 - facing * Vec3::Z * 4.0)
                .looking_at(road + facing * Vec3::Z * 60.0 + Vec3::Y * 3.2, Vec3::Y);
        }
        if frame == 2260 {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshots/world-city-road-height.png"));
        }
    }
    if frame == 2290 {
        survey.benchmark_passed = metrics.p95_ms <= 16.7;
        assert_eq!(
            buildings.iter().count(),
            layout.all_buildings().count(),
            "lost logical building root"
        );
        let report = serde_json::json!({"metrics":metrics.evidence(&clock),"resolution":windows.single().ok().map(|w|[w.resolution.physical_width(),w.resolution.physical_height()]),"present_mode":"AutoNoVsync","update_mode":"Continuous focused and unfocused (benchmark only)","window_focused":windows.single().ok().map(|w|w.focused),"visible_near_parts":lods.iter().filter(|(l,v)|l.near&&v.get()).count(),"visible_far_proxies":lods.iter().filter(|(l,v)|!l.near&&v.get()).count(),"benchmark":"warmed city view with active fifteen-racer simulation; current default rendering, no production quality preset", "logical_buildings":buildings.iter().count(),"junctions":layout.settings.junctions.len(),"fields":layout.fields.len(),"sections_captured":15,"p95_target_ms":16.7,"p95_target_met":metrics.p95_ms<=16.7,"limitations":"bounded wall-frame sample, not GPU timing or Windows hardware certification"});
        std::fs::write(
            "screenshots/world-survey.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .expect("write native world survey");
        println!(
            "World survey: 15 sections, {} roots, 1080p city p95 {:.2} ms (target <=16.7)",
            buildings.iter().count(),
            metrics.p95_ms
        );
    }
    if (2320..2680).contains(&frame) {
        race.phase = Phase::Paused;
        let pose = ((frame - 2320) / 40) as usize;
        let lean = if pose < 7 {
            (pose as f32 - 3.0) / 3.0
        } else {
            0.0
        };
        let (p, q) = track.sample(4160.0, 0.0);
        for (mut rider, mut state, mut t, mut history) in &mut riders {
            *t = Transform::from_translation(p - q * Vec3::Z * (rider.id as f32 * 50.0))
                .with_rotation(q);
            history.previous = *t;
            history.current = *t;
            state.lean = rr_core::steering::visual_bank(lean);
            rider.lean = state.lean;
            rider.kicking = pose >= 7;
            rider.attack = if pose >= 7 { 0.48 } else { 0.0 };
            rider.attack_side = if pose == 7 { -1.0 } else { 1.0 };
        }
        if let Ok(mut t) = camera.single_mut() {
            *t = Transform::from_translation(p - q * Vec3::Z * 6.0 + Vec3::Y * 2.5)
                .looking_at(p + Vec3::Y * 1.1, Vec3::Y);
        }
        if (frame - 2320) % 40 == 30 {
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(format!(
                    "screenshots/world-rider-pose-{pose}.png"
                )));
        }
    }
    if frame == 2700 {
        exit.write(if survey.benchmark_passed {
            AppExit::Success
        } else {
            AppExit::error()
        });
    }
}
