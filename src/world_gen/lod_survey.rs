//! Reproducible actual-render forward/reverse/orbit LOD review. No save writes.
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    max_cached: usize,
    max_panels: usize,
    max_triangles: usize,
    max_transitions: usize,
}
#[allow(clippy::too_many_arguments)]
pub fn run(
    mut commands: Commands,
    mut survey: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    mut race: ResMut<crate::game::Race>,
    mut cameras: Query<&mut Transform, With<crate::game::ChaseCamera>>,
    architecture: Option<Res<super::architecture::Architecture>>,
    stream: Option<Res<super::architecture::render::Streaming>>,
    identity: Option<Res<super::architecture::identity::Identity>>,
    city: Option<Res<super::city::visuals::ModelCatalog>>,
    mut exit: MessageWriter<AppExit>,
    mut ui: Query<&mut Visibility, With<Node>>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    for mut visibility in &mut ui {
        *visibility = Visibility::Hidden;
    }
    survey.frame += 1;
    let frame = survey.frame;
    race.phase = crate::game::Phase::Paused;
    let city_mode = city.is_some();
    let name = if city_mode { "city" } else { "valley" };
    let out = format!("screenshots/lod-{name}");
    if frame == 1 {
        std::fs::create_dir_all(&out).unwrap();
    }
    let eye_target = if let Some(plan) = &architecture {
        let b = plan
            .buildings
            .iter()
            .filter(|b| {
                plan.catalog.records[b.design].asset_type == "facade"
                    && (6.0..20.0).contains(&b.size.y)
            })
            .min_by(|a, b| a.size.y.total_cmp(&b.size.y))
            .expect("facade survey target");
        let phase = (frame % 360) as f32 / 360.0;
        if frame <= 360 || frame > 720 {
            let distance = 5.0 + 190.0 * (phase * 2.0 - 1.0).abs();
            let face = b.position + b.rotation * Vec3::new(0.0, 3.0, b.size.z * 0.5);
            (face + b.rotation * Vec3::Z * distance, face)
        } else {
            let angle = phase * std::f32::consts::TAU;
            let radius = b.size.x.hypot(b.size.z) * 0.5 + 15.0;
            (
                b.position
                    + b.rotation * Vec3::new(angle.sin() * radius, 4.0, angle.cos() * radius),
                b.position + Vec3::Y * 4.0,
            )
        }
    } else {
        let phase = (frame % 360) as f32 / 360.0;
        let z = 2050.0 + 220.0 * (phase * 2.0 - 1.0).abs();
        let backwards = (360..720).contains(&frame);
        (
            Vec3::new(1.75, 3.0, z),
            Vec3::new(1.75, 3.0, z + if backwards { -80.0 } else { 80.0 }),
        )
    };
    if let Ok(mut camera) = cameras.single_mut() {
        *camera = Transform::from_translation(eye_target.0).looking_at(eye_target.1, Vec3::Y);
    }
    if let (Some(s), Some(plan)) = (&stream, &architecture) {
        assert!(
            s.cached_lods() <= 48
                && s.active_panels <= plan.rules.max_active_panels
                && s.active_triangles <= plan.rules.max_active_triangles
        );
        survey.max_cached = survey.max_cached.max(s.cached_lods());
        survey.max_panels = survey.max_panels.max(s.active_panels);
        survey.max_triangles = survey.max_triangles.max(s.active_triangles);
        assert!(identity.as_ref().is_some_and(|i| i.ready()));
    }
    if let Some(c) = &city {
        assert!(c.cached_high_lods() <= 48 && c.transition_groups <= 8);
        survey.max_cached = survey.max_cached.max(c.cached_high_lods());
        survey.max_transitions = survey.max_transitions.max(c.transition_groups);
    }
    if frame.is_multiple_of(90) && frame <= 990 {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("{out}/{frame:04}.png")));
    }
    if frame == 1080 {
        let changes = stream.as_ref().map_or_else(
            || city.as_ref().unwrap().direct_crossfades,
            |s| s.direct_switches,
        );
        assert!(
            changes > 0,
            "test did not exercise a ready-to-ready handoff"
        );
        let report = serde_json::json!({"map":name,"render_frames":frame,"direct_crossfades":changes,"max_cached_lods":survey.max_cached,"max_displayed_panels":survey.max_panels,"max_displayed_triangles":survey.max_triangles,"max_city_transition_groups":survey.max_transitions,"views":"continuous near/far sweep, reverse review and Valley all-face orbit","saves":false,"limits":"captures and source/coverage checks do not imply perfect parallax, zero silhouette change or hardware/performance approval"});
        std::fs::write(
            format!("{out}/report.json"),
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .unwrap();
        println!("LOD survey passed: {name}, {changes} direct crossfades");
        exit.write(AppExit::Success);
    }
}
