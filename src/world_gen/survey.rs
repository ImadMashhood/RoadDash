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
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
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
    (architecture, streaming, slots, children, meshes, physics): (
        Option<Res<super::architecture::Architecture>>,
        Option<Res<super::architecture::render::Streaming>>,
        Query<&super::architecture::render::Slot>,
        Query<&Children>,
        Query<(), With<Mesh3d>>,
        Query<(), With<bevy_rapier3d::prelude::Collider>>,
    ),
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
        let architecture_evidence=architecture.as_ref().zip(streaming.as_ref()).map(|(a,s)| {
            assert!(s.active_buildings<=a.rules.max_active_buildings && s.active_panels<=a.rules.max_active_panels && s.active_triangles<=a.rules.max_active_triangles && s.cached_lods()<=48);
            let mut actual_meshes=0;let mut scene_roots=0;let mut prepared=0;
            for slot in &slots {
                scene_roots+=slot.scene_roots().len();prepared+=slot.prepared_roots().len();
                let mut stack=slot.scene_roots().to_vec();
                while let Some(e)=stack.pop() {
                    assert!(!physics.contains(e),"decorative building geometry acquired a collider");
                    actual_meshes+=usize::from(meshes.contains(e));
                    if let Ok(c)=children.get(e) {stack.extend(c.iter());}
                }
            }
            assert!(prepared<=a.rules.max_active_panels,"unbounded hidden preparations");
            assert!(actual_meshes>0,"architecture preview contains no instantiated GLB geometry");
            assert_eq!(scene_roots,s.active_panels,"streaming census differs from actual scene roots");
            serde_json::json!({"bound_lots":a.buildings.len(),"active_buildings":s.active_buildings,"active_panels":s.active_panels,"scene_roots":scene_roots,"prepared_panels":prepared,"actual_mesh_entities":actual_meshes,"visible_triangle_instances":s.active_triangles,"cached_lods":s.cached_lods(),"resident_coarse_lods":s.resident_coarse_lods(),"switches":s.switches,"catalog_designs":a.catalog.records.len(),"limits":{"buildings":a.rules.max_active_buildings,"panels":a.rules.max_active_panels,"triangles":a.rules.max_active_triangles}})
        });
        let report = serde_json::json!({"architecture":architecture_evidence,"metrics":metrics.evidence(&clock),"resolution":windows.single().ok().map(|w|[w.resolution.physical_width(),w.resolution.physical_height()]),"present_mode":"AutoNoVsync","update_mode":"Continuous focused and unfocused (benchmark only)","window_focused":windows.single().ok().map(|w|w.focused),"visible_near_parts":lods.iter().filter(|(l,v)|l.near&&v.get()).count(),"visible_far_proxies":lods.iter().filter(|(l,v)|!l.near&&v.get()).count(),"benchmark":"warmed city view with active fifteen-racer simulation; current default rendering, no production quality preset", "logical_buildings":buildings.iter().count(),"junctions":layout.settings.junctions.len(),"fields":layout.fields.len(),"sections_captured":15,"p95_target_ms":16.7,"p95_target_met":metrics.p95_ms<=16.7,"limitations":"bounded wall-frame sample, not GPU timing or Windows hardware certification"});
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
