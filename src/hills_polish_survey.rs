//! Active native evidence for the Hills polish: real wheel dust and moving mounts.
use crate::{
    game::{ChaseCamera, Phase, Race, Rider},
    track::Track,
    world_gen::hills::{
        atmosphere::{Atmosphere, Bird, DUST_CAPACITY, DustPuff},
        dressing::{DressingCensus, HillsDressing},
        mounted::{COUNT, Mounted},
    },
};
use bevy::{
    prelude::*,
    render::view::screenshot::{Screenshot, ScreenshotCaptured, save_to_disk},
};
use bevy_rapier3d::prelude::*;
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    captures: usize,
    maximum_dust: usize,
    first_mount: Option<Vec3>,
    moved: f32,
    mounted_collision_hit: bool,
    pause_clock: Option<f32>,
    paused_mount: Option<Vec3>,
    start: Option<Vec3>,
    travel: f32,
}
fn captured(_: On<ScreenshotCaptured>, mut state: ResMut<Survey>) {
    state.captures += 1;
}
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn run(
    mut commands: Commands,
    mut state: ResMut<Survey>,
    screen: Res<State<crate::runtime::AppScreen>>,
    track: Res<Track>,
    mut race: ResMut<Race>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    atmosphere: Res<Atmosphere>,
    dressing: Res<DressingCensus>,
    mut objects: Query<(
        Entity,
        &mut Transform,
        Option<&mut Rider>,
        Option<&Mounted>,
        Option<&Bird>,
        Option<&DustPuff>,
        Option<&HillsDressing>,
        Option<&ChaseCamera>,
        Option<&mut Velocity>,
        Option<&Collider>,
    )>,
    ui: Query<(Entity, Option<&ChildOf>), With<Node>>,
    mut exit: MessageWriter<AppExit>,
    physics: ReadRapierContext,
) {
    if matches!(
        screen.get(),
        crate::runtime::AppScreen::Boot | crate::runtime::AppScreen::Loading
    ) {
        return;
    }
    state.frame += 1;
    let frame = state.frame;
    for (entity, parent) in &ui {
        if parent.is_none() {
            commands.entity(entity).insert(Visibility::Hidden);
        }
    }
    let mut player = Vec3::ZERO;
    let mut mount = None;
    let mut mount_entity = None;
    let mut bird = None;
    let mut cactus = None;
    let mut mounts = 0;
    let mut birds = 0;
    let mut puffs = 0;
    for (entity, mut t, rider, mounted, flying, dust, detail, _, velocity, collider) in &mut objects
    {
        if let Some(mut r) = rider
            && r.id == 0
        {
            if frame == 1 {
                r.route = 0;
                r.progress = 9050.;
                r.lane = 12.;
                r.speed = 12.;
                let (p, q) = track.sample_route(0, r.progress, r.lane);
                *t = Transform::from_translation(p + Vec3::Y * 0.5).with_rotation(q);
                r.motion_velocity = q * Vec3::Z * 12.;
                if let Some(mut v) = velocity {
                    *v = Velocity::linear(r.motion_velocity);
                }
                state.start = Some(t.translation);
            }
            player = t.translation;
            state.travel = state.travel.max(state.start.unwrap().distance(player));
        }
        if let Some(m) = mounted {
            mounts += 1;
            assert!(collider.is_some(), "mounted pair lost its physical body");
            if m.index == 0 {
                mount = Some(t.translation);
                mount_entity = Some(entity);
            }
        }
        if flying.is_some() {
            birds += 1;
            if bird.is_none() {
                bird = Some(t.translation);
            }
        }
        if dust.is_some() {
            puffs += 1;
            assert!(collider.is_none());
        }
        if detail.is_some_and(|d| d.kind == "cactus") && cactus.is_none() {
            cactus = Some(t.translation);
        }
    }
    assert_eq!(mounts, COUNT);
    assert_eq!(birds, 24);
    assert_eq!(puffs, DUST_CAPACITY);
    let mount = mount.unwrap();
    if frame == 400 {
        let context = physics.single().expect("mounted survey physics context");
        let target = mount_entity.unwrap();
        let only_mount = |e, _: &bevy_rapier3d::rapier::geometry::Collider| e == target;
        state.mounted_collision_hit = context
            .cast_ray(
                mount + Vec3::Y * 1.35 + Vec3::X * 5.,
                Vec3::NEG_X,
                10.,
                true,
                QueryFilter::default().predicate(&only_mount),
            )
            .is_some();
        assert!(
            state.mounted_collision_hit,
            "walking mount collider failed actual native ray contact"
        );
    }
    if frame == 1 {
        state.first_mount = Some(mount);
    }
    state.moved = state.moved.max(mount.distance(state.first_mount.unwrap()));
    state.maximum_dust = state.maximum_dust.max(atmosphere.active_dust);
    assert!(atmosphere.active_dust <= DUST_CAPACITY);
    if frame < 840 {
        race.phase = Phase::Racing;
        if frame < 200 {
            keys.press(KeyCode::KeyW);
        } else {
            keys.release(KeyCode::KeyW);
        }
    } else {
        race.phase = Phase::Paused;
        keys.release(KeyCode::KeyW);
        if frame == 850 {
            state.pause_clock = Some(atmosphere.elapsed);
            state.paused_mount = Some(mount);
        }
        if frame > 850 {
            assert_eq!(Some(atmosphere.elapsed), state.pause_clock);
            assert!(mount.distance(state.paused_mount.unwrap()) < 0.0001);
        }
    }
    let (eye, target) = if frame < 220 {
        (player + Vec3::new(7., 4., -12.), player + Vec3::Y)
    } else if frame < 490 {
        let q = crate::world_gen::hills::mounted::pose(&track, 0, race.elapsed).rotation;
        let side = if frame < 380 { 6. } else { -6. };
        (mount + q * Vec3::new(side, 3., 7.), mount + Vec3::Y * 1.5)
    } else if frame < 610 {
        let p = cactus.unwrap();
        (p + Vec3::new(8., 5., -12.), p + Vec3::Y * 1.7)
    } else if frame < 730 {
        let (p, q) = track.sample(6200., -4.);
        (p + Vec3::Y * 4., p + q * Vec3::X * 22. + Vec3::Y * 4.)
    } else {
        let p = bird.unwrap();
        (p + Vec3::new(9., 1., -14.), p)
    };
    for (_, mut t, _, _, _, _, _, camera, _, _) in &mut objects {
        if camera.is_some() {
            *t = Transform::from_translation(eye).looking_at(target, Vec3::Y);
        }
    }
    if let Some(name) = match frame {
        100 => Some("dust-early"),
        180 => Some("dust-trail"),
        320 => Some("mounted-a"),
        440 => Some("mounted-b"),
        560 => Some("cactus"),
        680 => Some("western-detail"),
        800 => Some("birds-lighting"),
        _ => None,
    } {
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(format!("screenshots/polish-{name}.png")))
            .observe(captured);
    }
    if frame > 930 && state.captures == 7 {
        assert!(
            atmosphere.emitted > 0 && state.maximum_dust > 0,
            "real unpaved riding emitted no dust"
        );
        assert!(state.travel > 5., "actual motorcycle failed to travel");
        assert!(state.moved > 1., "mounted pair did not patrol");
        assert!(dressing.cacti > 0 && dressing.rocks > 0 && dressing.deadwood > 0);
        std::fs::write("screenshots/hills-polish.json", serde_json::to_vec_pretty(&serde_json::json!({
            "captures_completed":state.captures,"dust_emitted":atmosphere.emitted,"maximum_active_dust":state.maximum_dust,
            "dust_capacity":DUST_CAPACITY,"bike_displacement_m":state.travel,"mounted_pairs":mounts,"mounted_displacement_m":state.moved,"mounted_collision_hit":state.mounted_collision_hit,
            "birds":birds,"pause_frames_verified":frame-850,"dressing":&*dressing,
            "limits":"Scripted active evidence; not a human full race, target GPU benchmark or production animal AI/ejection certification."
        })).unwrap()).unwrap();
        exit.write(AppExit::Success);
    }
    assert!(frame < 1100, "polish captures timed out");
}
