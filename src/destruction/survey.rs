//! Reproducible native fixture using the production registration/damage systems.
use super::*;
use bevy::render::view::screenshot::{Screenshot, save_to_disk};
#[derive(Resource, Default)]
pub struct Survey {
    frame: u32,
    props: Vec<Entity>,
    base: Vec3,
    peak: usize,
}
#[derive(Component)]
pub struct Probe;
#[allow(clippy::too_many_arguments, clippy::type_complexity)]
pub fn step(
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
    state.frame += 1;
    race.phase = Phase::Racing;
    for (e, parent) in &ui {
        if parent.is_none() {
            commands.entity(e).insert(Visibility::Hidden);
        }
    }
    if state.frame == 1 {
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
        assert_eq!(forbidden.iter().count(), 0);
        assert_eq!(props.iter().count(), 1008);
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
        }
        std::fs::write("screenshots/destruction-results.json",format!("{{\"production_props\":995,\"tested_kinds\":13,\"real_collision_breaks\":13,\"restart_restored\":true,\"cars_damaged\":0,\"peak_dynamic_props\":{},\"frame\":{}}}",state.peak,state.frame)).unwrap();
        exit.write(AppExit::Success);
    }
}
pub fn camera(
    state: Res<Survey>,
    props: Query<&Transform, (With<Breakable>, Without<Camera3d>)>,
    mut cameras: Query<&mut Transform, With<Camera3d>>,
) {
    if state.frame == 0 {
        return;
    }
    let mut lo = state.base;
    let mut hi = state.base;
    for e in &state.props {
        if let Ok(t) = props.get(*e) {
            lo = lo.min(t.translation);
            hi = hi.max(t.translation);
        }
    }
    let center = (lo + hi) * 0.5;
    let width = (hi.x - lo.x).max(38.);
    for mut t in &mut cameras {
        *t = Transform::from_translation(center + Vec3::new(8., width * 0.65, -width * 0.85))
            .looking_at(center + Vec3::Y, Vec3::Y);
    }
}
