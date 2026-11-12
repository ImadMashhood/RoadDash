//! Native solid-geometry receipt. The diagnostic walker uses the production
//! capsule/controller and moves through Rapier, never by rewriting its transform.
use super::*;
use crate::game::{Phase, Race};
use bevy::render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    ticks: u32,
    walker: Option<Entity>,
    targets: Vec<Vec3>,
    reached: usize,
    grounded: u32,
    captures: usize,
    views: usize,
}
#[derive(Component)]
pub struct Walker;
fn captured(_: On<ScreenshotCaptured>, mut survey: ResMut<Survey>) {
    survey.captures += 1;
}
#[allow(clippy::too_many_arguments)]
pub fn setup(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    runs: Option<Res<RailRuns>>,
    art: Res<Art>,
    mut race: ResMut<Race>,
    context: ReadRapierContext,
    ground: Query<(), With<crate::world::Ground>>,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    race.phase = Phase::Racing;
    if state.frame < 10 || state.walker.is_some() {
        return;
    }
    let Some(runs) = runs else {
        return;
    };
    let pair = runs
        .0
        .windows(2)
        .find(|p| p[0].side == p[1].side && (p[1].start - p[0].end - 16.).abs() < 0.01)
        .expect("real re-entry gap");
    let s = (pair[0].end + pair[1].start) * 0.5;
    let ctx = context.single().expect("native physics");
    let is_ground = |e, _: &bevy_rapier3d::rapier::geometry::Collider| ground.contains(e);
    let sample = |lane| {
        let p = track.sample(s, lane).0;
        let (_, toi) = ctx
            .cast_ray(
                p + Vec3::Y * 30.,
                -Vec3::Y,
                60.,
                true,
                QueryFilter::default().predicate(&is_ground),
            )
            .expect("gap walking support");
        p.with_y(p.y + 30. - toi + 0.98)
    };
    let outside = sample(pair[0].side as f32 * 12.);
    let inside = sample(pair[0].side as f32 * 4.);
    let (walker, _) = crate::world::runner(&mut commands, &art, 0);
    commands.entity(walker).insert((
        Walker,
        Transform::from_translation(outside),
        RigidBody::KinematicPositionBased,
        Collider::capsule_y(0.65, 0.28),
        crate::recovery::runner_controller(),
    ));
    state.walker = Some(walker);
    state.targets = vec![inside, outside];
}
pub fn walk(
    mut state: ResMut<Survey>,
    mut walkers: Query<
        (
            &Transform,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Walker>,
    >,
) {
    let Some(target) = state.targets.get(state.reached).copied() else {
        return;
    };
    let Ok((pose, mut controller, output)) = walkers.single_mut() else {
        return;
    };
    state.ticks += 1;
    if output.is_some_and(|o| o.grounded) {
        state.grounded += 1;
    }
    let delta = (target - pose.translation).with_y(0.);
    if delta.length() < 0.25 {
        state.reached += 1;
    }
    let down = crate::recovery::runner_downward_speed(output.is_some_and(|o| o.grounded));
    controller.translation = Some(delta.clamp_length_max(3. / 60.) - Vec3::Y * (down / 60.));
    assert!(
        state.ticks < 1800,
        "native foot traversal failed without rescue: {:?}",
        pose.translation
    );
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn camera(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    track: Res<Track>,
    runs: Option<Res<RailRuns>>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
    walkers: Query<&Transform, (With<Walker>, Without<Camera3d>)>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    mut exit: MessageWriter<AppExit>,
) {
    let Some(runs) = runs else {
        return;
    };
    for (entity, parent) in &ui {
        if parent.is_none() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    let Ok(walker) = walkers.single() else {
        return;
    };
    let target = if state.reached < 2 {
        walker.translation
    } else {
        let side = if state.views.is_multiple_of(2) { -1 } else { 1 };
        let r = runs.0.iter().find(|r| r.side == side).unwrap();
        track
            .sample(
                r.start + 2.,
                side as f32 * (track.half_width(r.start) + 1.8),
            )
            .0
            + Vec3::Y * 0.8
    };
    for mut camera in &mut cameras {
        *camera = Transform::from_translation(target + Vec3::new(9., 4., -10.))
            .looking_at(target, Vec3::Y);
    }
    if state.frame.is_multiple_of(90) && state.views < 4 && (state.views < 2 || state.reached == 2)
    {
        let index = state.views;
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!(
                "screenshots/guardrails-{index:02}.png"
            )))
            .observe(captured);
        state.views += 1;
    }
    if state.reached == 2 && state.captures == 4 {
        assert!(
            state.grounded > 30,
            "walker must traverse real supporting surfaces"
        );
        let report = serde_json::json!({"completed":true,"physical_foot_crossings":state.reached,"grounded_ticks":state.grounded,"fixed_ticks":state.ticks,"rescues":0,"run_count":runs.0.len(),"max_run_m":policy::MAX_RUN_M,"gap_m":policy::GAP_M,"screenshots":state.captures});
        std::fs::create_dir_all("screenshots").expect("survey directory");
        std::fs::write(
            "screenshots/guardrails-report.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .expect("survey receipt");
        info!("GUARDRAIL_SURVEY_COMPLETE {report}");
        exit.write(AppExit::Success);
    }
}
