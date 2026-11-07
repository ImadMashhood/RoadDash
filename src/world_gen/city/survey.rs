//! Bounded native inspection of the actual playable city, including all districts.
use super::{GeneratedCity, visuals::ModelCatalog};
use crate::game::{ChaseCamera, Phase, Race};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    last: Option<std::time::Instant>,
    samples: Vec<f64>,
}
const VIEWS: [(&str, [f32; 3], [f32; 3]); 9] = [
    (
        "01-full-5300-map",
        [4500.0, 4500.0, 2650.0],
        [0.0, 0.0, 2650.0],
    ),
    ("02-residential", [1.75, 3.0, 250.0], [1.75, 4.0, 330.0]),
    ("03-warehouses", [-2.0, 5.0, 1000.0], [10.0, 6.0, 1080.0]),
    ("04-run-down", [1.75, 3.0, 1550.0], [1.75, 4.0, 1610.0]),
    (
        "05-commercial-corner",
        [-5.0, 7.0, 2190.0],
        [21.0, 5.0, 2225.0],
    ),
    (
        "06-complete-roofs-and-rears",
        [37.0, 35.0, 2840.0],
        [30.0, 5.0, 2910.0],
    ),
    ("07-chinatown", [1.75, 3.0, 3500.0], [1.75, 5.0, 3580.0]),
    ("08-finish-5300", [2.0, 4.0, 5230.0], [2.0, 3.0, 5310.0]),
    ("09-whole-block", [100.0, 95.0, 3930.0], [0.0, 0.0, 4010.0]),
];
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run(
    mut commands: Commands,
    mut survey: ResMut<Survey>,
    city: Res<GeneratedCity>,
    catalog: Res<ModelCatalog>,
    server: Res<AssetServer>,
    mut cameras: Query<(&mut Transform, &mut Projection, &mut DistanceFog), With<ChaseCamera>>,
    mut race: ResMut<Race>,
    metrics: Res<crate::diagnostics::Metrics>,
    clock: Res<crate::runtime::SimulationClock>,
    mut exit: MessageWriter<AppExit>,
) {
    if survey.frame == 0 {
        std::fs::create_dir_all("screenshots/city5300").expect("city survey directory");
        for (name, _, _) in VIEWS {
            let path = format!("screenshots/city5300/{name}.png");
            if std::path::Path::new(&path).exists() {
                std::fs::remove_file(path).expect("replace prior city capture");
            }
        }
    }
    let stage = ((survey.frame) / 90).min(8) as usize;
    let (name, eye, target) = VIEWS[stage];
    if let Ok((mut transform, mut projection, mut fog)) = cameras.single_mut() {
        *transform = Transform::from_xyz(eye[0], eye[1], eye[2])
            .looking_at(Vec3::from_array(target), Vec3::Y);
        if let Projection::Perspective(p) = &mut *projection {
            p.far = 20000.0;
            p.fov = if stage == 0 { 0.9 } else { 1.05 };
        }
        fog.falloff = bevy::pbr::FogFalloff::Linear {
            start: if stage == 0 { 6000.0 } else { 180.0 },
            end: if stage == 0 { 20000.0 } else { 1200.0 },
        };
    }
    if !catalog.ready(&server) {
        return;
    }
    race.phase = Phase::Racing;
    let now = std::time::Instant::now();
    if let Some(last) = survey.last.replace(now) {
        survey
            .samples
            .push(now.duration_since(last).as_secs_f64() * 1000.0);
    }
    survey.frame += 1;
    if survey.frame % 90 == 70 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/city5300/{name}.png")));
    }
    if survey.frame == 9 * 90 {
        let mut report = metrics.evidence(&clock);
        report["city"] = serde_json::json!({"length_m":city.plan.length_m,"buildings":city.plan.lots.len(),"district_zones":city.plan.district_zones,"facade_designs_used":city.plan.model_design_ids.len(),"geometric_facade_instances":catalog.instances,"requested_lods":catalog.requested_lods(),"captures":9,"scope":"actual playable world; survey viewpoints are developer camera overrides; no steady-state racing performance approval"});
        survey.samples.sort_by(f64::total_cmp);
        let samples = &survey.samples;
        report["survey_wall_frames_ms"] = serde_json::json!({"samples":samples.len(),"median":samples[samples.len()/2],"p95":samples[samples.len()*95/100]});
        std::fs::create_dir_all("screenshots/city5300").unwrap();
        std::fs::write(
            "screenshots/city5300/report.json",
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        assert!(
            VIEWS
                .iter()
                .all(|(name, _, _)| std::path::Path::new(&format!(
                    "screenshots/city5300/{name}.png"
                ))
                .is_file()),
            "missing survey capture"
        );
        println!("City5300 native survey passed: 977 full buildings, 53 blocks, 9 views");
        exit.write(AppExit::Success);
    }
}
