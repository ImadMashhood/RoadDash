//! Opt-in native fixture. Scenario placement is never installed in normal play.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    phase: u8,
    started: f32,
    states: std::collections::BTreeSet<String>,
    foot: bool,
    falling: bool,
    done: u32,
    occluder: Option<Entity>,
    bust_metrics: Option<serde_json::Value>,
    bust_police: Option<serde_json::Value>,
}
pub fn observe(
    mut survey: ResMut<Survey>,
    police: Res<Police>,
    race: Res<Race>,
    riders: Query<(&Rider, &Recovery)>,
) {
    survey.states.insert(format!("{:?}", police.mind.state));
    for (r, rec) in &riders {
        if r.police && rec.stage == RecoveryStage::Running {
            survey.foot = true;
        }
        if r.id == 0 && rec.stage == RecoveryStage::Falling {
            survey.falling = true;
            assert!(!race.arrested, "arrest during falling ragdoll");
        }
    }
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run(
    mut survey: ResMut<Survey>,
    mut commands: Commands,
    screen: Res<bevy::prelude::State<crate::runtime::AppScreen>>,
    mut race: ResMut<Race>,
    mut police: ResMut<Police>,
    mut riders: Query<(Entity, &mut Rider, &mut Recovery, &mut Transform)>,
    track: Res<Track>,
    art: Res<Art>,
    context: ReadRapierContext,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut tuning: ResMut<crate::recovery::RecoveryTuning>,
    metrics: Res<crate::diagnostics::Metrics>,
    clock: Res<crate::runtime::SimulationClock>,
    profile: Res<crate::game::Profile>,
    mut exit: MessageWriter<AppExit>,
) {
    survey.frame += 1;
    assert!(
        survey.frame < 5000,
        "bounded police native fixture timed out: phase={} states={:?} mind={:?}",
        survey.phase,
        survey.states,
        police.mind
    );
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    if survey.phase == 0 {
        race.phase = Phase::Racing;
        race.elapsed = 10.;
        for (e, mut r, _, mut t) in &mut riders {
            r.progress = if r.police {
                306.
            } else if r.id == 0 {
                300.
            } else {
                40. - r.id as f32 * 12.
            };
            r.route = 0;
            r.lane = if r.police { 0. } else { -2. };
            r.immunity = 0.;
            let (p, q) = track.sample_route(0, r.progress, r.lane);
            *t = Transform::from_translation(p).with_rotation(q);
            crate::vehicle::remount_at(&mut commands, e, *t);
        }
        keys.press(KeyCode::KeyW);
        keys.press(KeyCode::ShiftLeft);
        survey.phase = 1;
        survey.started = race.elapsed;
        println!("Police native: normal input speeding / warning / pursuit / escape scenario");
    } else if survey.phase == 1 {
        assert!(
            race.elapsed - survey.started < 65.,
            "speeding scenario never escaped: {:?}",
            police.mind
        );
        if police.mind.state.wanted() && survey.occluder.is_none() {
            let (_, pr, _, pt) = riders.iter().find(|(_, r, _, _)| r.id == 0).unwrap();
            let (_, cr, _, ct) = riders.iter().find(|(_, r, _, _)| r.police).unwrap();
            if pr.progress > cr.progress + 20. {
                let midpoint = (pt.translation + ct.translation) * 0.5 + Vec3::Y * 4.;
                let direction = (pt.translation - ct.translation).with_y(0.).normalize();
                let rotation = Quat::from_rotation_arc(Vec3::Z, direction);
                let wall = commands
                    .spawn((
                        Name::new("Police survey temporary occlusion wall"),
                        RigidBody::Fixed,
                        Collider::cuboid(20., 4., 0.2),
                        Mesh3d(art.cube.clone()),
                        MeshMaterial3d(art.dark.clone()),
                        Transform::from_translation(midpoint)
                            .with_rotation(rotation)
                            .with_scale(Vec3::new(40., 8., 0.4)),
                    ))
                    .id();
                // Collider must remain unscaled relative to its authored dimensions.
                commands
                    .entity(wall)
                    .insert(Collider::cuboid(0.5, 0.5, 0.5));
                survey.occluder = Some(wall);
                println!(
                    "Police native: temporary solid LOS barrier behind moving player; wait for real loss/search/escape"
                );
            }
        }
        if police.escapes > 0 {
            if let Some(wall) = survey.occluder.take() {
                commands.entity(wall).despawn();
            }
            assert!(
                survey.states.contains("Warning")
                    && survey.states.contains("Pursue")
                    && survey.states.contains("Search")
            );
            keys.release(KeyCode::KeyW);
            keys.release(KeyCode::ShiftLeft);
            crate::verification::retry_key(&mut commands, true);
            survey.phase = 10;
        }
    } else if survey.phase == 10
        && race.phase == Phase::Countdown
        && police.mind.state == State::Patrol
    {
        crate::verification::retry_key(&mut commands, false);
        race.phase = Phase::Racing;
        race.elapsed = 10.;
        for (e, mut r, _, mut t) in &mut riders {
            r.progress = if r.police {
                508.
            } else if r.id == 0 {
                500.
            } else {
                40. - r.id as f32 * 12.
            };
            r.lane = if r.police { 0. } else { -2. };
            r.speed = 0.;
            r.motion_velocity = Vec3::ZERO;
            r.immunity = 0.;
            let (p, q) = track.sample_route(0, r.progress, r.lane);
            *t = Transform::from_translation(p).with_rotation(q);
            crate::vehicle::remount_at(&mut commands, e, *t);
        }
        *police = Police::default();
        police.mind.state = State::Pursue;
        tuning.0.auto_seek = false;
        survey.phase = 2;
        survey.started = race.elapsed;
    } else if survey.phase == 2 {
        if race.elapsed - survey.started > 0.15 {
            let (e, mut r, mut rec, t) = riders.iter_mut().find(|(_, r, _, _)| r.id == 0).unwrap();
            r.immunity = 1.;
            crate::recovery::crash(
                &mut commands,
                &art,
                e,
                &mut r,
                &mut rec,
                t.translation,
                t.rotation,
                Vec3::ZERO,
                &context,
            );
            survey.phase = 3;
            survey.started = race.elapsed;
            println!("Police native: stopped crash; preserve falling/getup before foot arrest");
        }
    } else if survey.phase == 3 {
        assert!(
            race.elapsed - survey.started < 40.,
            "foot scenario did not arrest: {:?}",
            police.mind
        );
        if race.arrested {
            assert!(
                survey.foot && survey.falling,
                "missing actual foot/falling lifecycle"
            );
            assert!(
                riders
                    .iter()
                    .find(|(_, r, _, _)| r.id == 0)
                    .unwrap()
                    .1
                    .immunity
                    > 0.,
                "fixture did not preserve recent-remount immunity into foot grace test"
            );
            assert_eq!(profile.races, 0);
            assert_eq!(profile.best_seconds, None);
            std::fs::create_dir_all("docs/testing/part08-police/native")
                .expect("create police native evidence directory");
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("docs/testing/part08-police/native/busted.png"));
            survey.phase = 4;
            survey.done = survey.frame;
            println!(
                "Police native: physical foot chase and safe arrest observed; no race/save payout"
            );
        }
    } else if survey.phase == 4 && survey.frame > survey.done + 30 {
        assert!(
            std::path::Path::new("docs/testing/part08-police/native/busted.png").exists(),
            "screenshot completion missing"
        );
        survey.bust_metrics = Some(metrics.evidence(&clock));
        survey.bust_police = Some(
            serde_json::json!({"detection_perceptions":police.perceptions,"safe_contact_checks":police.contact_checks,"ray_state_evaluations":police.evaluations,"mean_evaluation_us":police.micros as f64/police.evaluations.max(1) as f64,"max_evaluation_us":police.max_micros,"scope":"timed conditional ray/state span; excludes common rider queries, drive, navigation, physics and rendering; no controlled whole-police A/B"}),
        );
        crate::verification::retry_key(&mut commands, true);
        survey.phase = 5;
    } else if survey.phase == 5
        && race.phase == Phase::Countdown
        && police.mind.state == State::Patrol
    {
        crate::verification::retry_key(&mut commands, false);
        assert!(!race.arrested);
        assert_eq!(police.mind.state, State::Patrol);
        assert!(
            riders
                .iter()
                .all(|(_, _, rec, _)| rec.stage == RecoveryStage::Riding)
        );
        let report = serde_json::json!({"states":survey.states,"actual_officer_running":survey.foot,"actual_player_ragdoll":survey.falling,"busted_observed":true,"native_retry_verified":true,"retry_arrested":race.arrested,"recent_remount_foot_grace_expired":true,"racer_count":riders.iter().filter(|(_,r,_,_)|!r.police).count(),"officers":riders.iter().filter(|(_,r,_,_)|r.police).count(),"police_cost":survey.bust_police,"profile_races":profile.races,"profile_best":profile.best_seconds,"metrics":survey.bust_metrics,"limits":"scripted Linux RTX3080 fixture; scenario placements only; not human balance, complete art, broad perf or hardware approval"});
        std::fs::write(
            "docs/testing/part08-police/native/police.json",
            serde_json::to_string_pretty(&report).unwrap(),
        )
        .unwrap();
        exit.write(AppExit::Success);
    }
}
