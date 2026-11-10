//! Original mounted-cowboy ambience. The host owns physical contacts; all peers
//! derive the same bounded kinematic pose from the replicated race clock.
//! These are scenic walking mounts, not a replacement for either accepted rider rig.
use super::{HillsCowboy, HillsHorse, HillsSolid, Track, props::Model};
use crate::{game::Race, world::Obstacle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const COUNT: usize = 5;
const SITES: [(usize, f32, f32); COUNT] = [
    (0, 5520., 39.),
    (1, 6300., -110.),
    (0, 6860., 110.),
    (1, 7000., -110.),
    (0, 8460., 39.),
];
#[derive(Component)]
pub struct Mounted {
    pub index: usize,
    last_elapsed: f32,
}
#[derive(Component)]
pub struct MountedLimb {
    index: usize,
    leg: usize,
    lower: bool,
}
fn center(track: &Track, index: usize) -> (Vec3, Quat) {
    let (route, station, lane) = SITES[index];
    let (mut p, q) = track.sample_route(route, station, 0.);
    p -= q * Vec3::X * lane;
    let forward = (q * Vec3::Z).with_y(0.).normalize();
    (p, Quat::from_rotation_y(forward.x.atan2(forward.z)))
}
/// Reserve the complete swept horse envelope, including its turning radius.
pub(super) fn reserved(track: &Track, point: Vec3, radius: f32) -> bool {
    (0..COUNT).any(|i| {
        let (p, q) = center(track, i);
        let local = q.inverse() * (point - p);
        local.x.abs() < 9. + radius && local.z.abs() < 17. + radius
    })
}
fn phase(index: usize, elapsed: f32) -> f32 {
    elapsed.max(0.) * 0.08 + index as f32 * 1.17
}
/// Absolute-clock evaluation means pause, restart and a late-joining guest do
/// not accumulate a different route phase. Road/shoulder collision is unchanged.
pub fn pose(track: &Track, index: usize, elapsed: f32) -> Transform {
    let (center, frame) = center(track, index);
    let a = phase(index, elapsed);
    let mut p = center + frame * Vec3::new(6. * a.cos(), 0., 14. * a.sin());
    p.y = super::geometry::ground_height(track, p.x, p.z);
    let direction = frame * Vec3::new(-6. * a.sin(), 0., 14. * a.cos());
    Transform::from_translation(p)
        .with_rotation(Quat::from_rotation_y(direction.x.atan2(direction.z)))
}
fn leg_rotation(index: usize, leg: usize, lower: bool, elapsed: f32) -> Quat {
    // Four-beat walking order. Knees flex during swing; planted legs extend.
    let offset = [
        0.,
        std::f32::consts::PI,
        1.5 * std::f32::consts::PI,
        0.5 * std::f32::consts::PI,
    ][leg];
    let stride = (elapsed.max(0.) * 5.8 + index as f32 + offset).sin();
    Quat::from_rotation_x(if lower {
        -0.55 * stride.max(0.)
    } else {
        0.25 * stride
    })
}
fn body(index: usize) -> Model {
    let mut m = Model::default();
    let hide = if index.is_multiple_of(2) {
        [0.35, 0.16, 0.075]
    } else {
        [0.64, 0.45, 0.25]
    };
    let dark = [0.085, 0.055, 0.035];
    let leather = [0.20, 0.095, 0.04];
    m.ellipsoid(Vec3::new(0., 1.35, 0.), Vec3::new(0.42, 0.46, 1.), hide, 0.);
    m.rod(
        Vec3::new(0., 1.45, 0.55),
        Vec3::new(0., 2.08, 0.85),
        0.27,
        hide,
    );
    m.ellipsoid(
        Vec3::new(0., 2.1, 1.08),
        Vec3::new(0.23, 0.24, 0.48),
        hide,
        0.,
    );
    m.ellipsoid(
        Vec3::new(0., 1.98, 1.46),
        Vec3::new(0.23, 0.17, 0.20),
        dark,
        0.,
    );
    for side in [-1., 1.] {
        m.rod(
            Vec3::new(side * 0.15, 2.25, 0.85),
            Vec3::new(side * 0.19, 2.56, 0.82),
            0.065,
            hide,
        );
        m.ellipsoid(
            Vec3::new(side * 0.218, 2.18, 1.1),
            Vec3::splat(0.035),
            [0.025, 0.02, 0.018],
            0.,
        );
    }
    m.rod(
        Vec3::new(0., 1.95, 0.67),
        Vec3::new(0., 1.45, 0.30),
        0.12,
        dark,
    );
    m.rod(
        Vec3::new(0., 1.55, -0.95),
        Vec3::new(0., 0.55, -1.2),
        0.10,
        dark,
    );
    m.ellipsoid(
        Vec3::new(0., 1.76, -0.10),
        Vec3::new(0.48, 0.14, 0.48),
        leather,
        0.,
    );
    // Seated cowboy silhouette: bent thighs around saddle, boots in stirrups,
    // upright shirt, bent elbows to reins, head and unmistakable broad brim.
    let shirt = if index.is_multiple_of(2) {
        [0.62, 0.19, 0.10]
    } else {
        [0.18, 0.36, 0.46]
    };
    let denim = [0.13, 0.18, 0.24];
    let skin = [0.66, 0.43, 0.28];
    m.ellipsoid(
        Vec3::new(0., 2.08, -0.13),
        Vec3::new(0.29, 0.36, 0.19),
        shirt,
        0.,
    );
    m.ellipsoid(
        Vec3::new(0., 2.62, -0.08),
        Vec3::new(0.15, 0.20, 0.15),
        skin,
        0.,
    );
    m.ellipsoid(
        Vec3::new(0., 2.82, -0.08),
        Vec3::new(0.37, 0.045, 0.29),
        [0.72, 0.56, 0.32],
        0.,
    );
    m.ellipsoid(
        Vec3::new(0., 2.91, -0.08),
        Vec3::new(0.18, 0.12, 0.16),
        [0.72, 0.56, 0.32],
        0.,
    );
    m.ellipsoid(
        Vec3::new(0., 2.62, 0.075),
        Vec3::new(0.035, 0.05, 0.045),
        skin,
        0.,
    );
    for side in [-1., 1.] {
        m.ellipsoid(
            Vec3::new(side * 0.055, 2.68, 0.053),
            Vec3::new(0.018, 0.014, 0.014),
            dark,
            0.,
        );
    }
    for side in [-1., 1.] {
        let hip = Vec3::new(side * 0.16, 1.85, -0.1);
        let knee = Vec3::new(side * 0.48, 1.52, 0.23);
        let ankle = Vec3::new(side * 0.46, 1.02, 0.12);
        m.rod(hip, knee, 0.12, denim);
        m.rod(knee, ankle, 0.095, denim);
        m.ellipsoid(
            ankle + Vec3::new(0., -0.02, 0.10),
            Vec3::new(0.12, 0.09, 0.22),
            leather,
            0.,
        );
        m.rod(
            Vec3::new(side * 0.27, 2.31, -0.1),
            Vec3::new(side * 0.33, 2.08, 0.15),
            0.085,
            shirt,
        );
        m.rod(
            Vec3::new(side * 0.33, 2.08, 0.15),
            Vec3::new(side * 0.17, 2.01, 0.48),
            0.065,
            shirt,
        );
        m.ellipsoid(
            Vec3::new(side * 0.17, 2.01, 0.48),
            Vec3::splat(0.075),
            skin,
            0.,
        );
        m.rod(
            Vec3::new(side * 0.17, 2.01, 0.48),
            Vec3::new(side * 0.24, 2.02, 1.32),
            0.016,
            leather,
        );
    }
    m
}
/// Called during world startup, before the first Rapier synchronization.
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.88,
        ..default()
    });
    for index in 0..COUNT {
        let root = commands
            .spawn((
                Name::new(format!("Hills mounted cowboy {index}")),
                Mounted {
                    index,
                    last_elapsed: 0.,
                },
                HillsHorse,
                HillsCowboy,
                HillsSolid,
                Obstacle,
                Mesh3d(meshes.add(body(index).mesh())),
                MeshMaterial3d(material.clone()),
                pose(track, index, 0.),
                RigidBody::KinematicPositionBased,
                Collider::compound(vec![
                    (
                        Vec3::new(0., 1.34, 0.),
                        Quat::IDENTITY,
                        Collider::cuboid(0.41, 0.44, 0.98),
                    ),
                    (
                        Vec3::new(0., 1.91, 0.85),
                        Quat::IDENTITY,
                        Collider::cuboid(0.25, 0.41, 0.55),
                    ),
                    (
                        Vec3::new(0., 2.35, -0.1),
                        Quat::IDENTITY,
                        Collider::capsule_y(0.33, 0.19),
                    ),
                ]),
                Friction::coefficient(0.65),
            ))
            .id();
        for leg in 0..4 {
            let side = if leg % 2 == 0 { -1. } else { 1. };
            let z = if leg < 2 { 0.64 } else { -0.64 };
            let hide = if index.is_multiple_of(2) {
                [0.35, 0.16, 0.075]
            } else {
                [0.64, 0.45, 0.25]
            };
            let mut upper = Model::default();
            upper.rod(Vec3::ZERO, -Vec3::Y * 0.48, 0.085, hide);
            let joint = commands
                .spawn((
                    MountedLimb {
                        index,
                        leg,
                        lower: false,
                    },
                    Mesh3d(meshes.add(upper.mesh())),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(side * 0.29, 1.02, z)
                        .with_rotation(leg_rotation(index, leg, false, 0.)),
                    Collider::compound(vec![(
                        -Vec3::Y * 0.24,
                        Quat::IDENTITY,
                        Collider::cuboid(0.085, 0.24, 0.085),
                    )]),
                ))
                .id();
            let mut lower = Model::default();
            lower.rod(Vec3::ZERO, -Vec3::Y * 0.43, 0.065, hide);
            lower.ellipsoid(
                Vec3::new(0., -0.43, 0.04),
                Vec3::new(0.12, 0.08, 0.17),
                [0.075, 0.05, 0.035],
                0.,
            );
            let hoof = commands
                .spawn((
                    MountedLimb {
                        index,
                        leg,
                        lower: true,
                    },
                    Mesh3d(meshes.add(lower.mesh())),
                    MeshMaterial3d(material.clone()),
                    Transform::from_xyz(0., -0.48, 0.)
                        .with_rotation(leg_rotation(index, leg, true, 0.)),
                    Collider::compound(vec![
                        (
                            -Vec3::Y * 0.22,
                            Quat::IDENTITY,
                            Collider::cuboid(0.07, 0.22, 0.08),
                        ),
                        (
                            Vec3::new(0., -0.43, 0.04),
                            Quat::IDENTITY,
                            Collider::cuboid(0.12, 0.08, 0.17),
                        ),
                    ]),
                ))
                .id();
            commands.entity(joint).add_child(hoof);
            commands.entity(root).add_child(joint);
        }
    }
}
/// Schedule in FixedUpdate after Gameplay and before PhysicsSet::SyncBackend.
/// This also runs for guests, whose race elapsed is supplied by host snapshots.
pub fn update(
    mut commands: Commands,
    race: Res<Race>,
    track: Res<Track>,
    mut roots: Query<(Entity, &mut Mounted, &mut Transform), Without<MountedLimb>>,
    mut limbs: Query<(&MountedLimb, &mut Transform), Without<Mounted>>,
) {
    for (entity, mut mounted, mut transform) in &mut roots {
        let next = pose(&track, mounted.index, race.elapsed);
        if race.elapsed < mounted.last_elapsed {
            crate::environment::teleport_kinematic(&mut commands, entity, next);
        }
        *transform = next;
        mounted.last_elapsed = race.elapsed;
    }
    for (limb, mut transform) in &mut limbs {
        transform.rotation = leg_rotation(limb.index, limb.leg, limb.lower, race.elapsed);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mounted_patrols_stay_grounded_clear_of_both_roads_and_inside_reserved_envelopes() {
        let track = Track::hills().unwrap();
        for i in 0..COUNT {
            for tick in 0..640 {
                let t = pose(&track, i, tick as f32 / 8.);
                assert!(reserved(&track, t.translation, 0.));
                assert!(
                    (t.translation.y
                        - super::super::geometry::ground_height(
                            &track,
                            t.translation.x,
                            t.translation.z
                        ))
                    .abs()
                        < 0.001
                );
                assert!(track.scenery_fits(t.translation, 2., 3.));
                let next = pose(&track, i, tick as f32 / 8. + 1. / 60.);
                assert!(next.translation.distance(t.translation) < 0.03);
            }
        }
    }
    #[test]
    fn clock_driven_mounts_pause_restart_and_match_late_guest_snapshots() {
        fn app() -> (App, Entity, Entity) {
            let mut app = App::new();
            app.insert_resource(Track::hills().unwrap())
                .insert_resource(Race::default())
                .add_systems(FixedUpdate, update);
            let root = app
                .world_mut()
                .spawn((
                    Mounted {
                        index: 2,
                        last_elapsed: 0.,
                    },
                    Transform::default(),
                ))
                .id();
            let limb = app
                .world_mut()
                .spawn((
                    MountedLimb {
                        index: 2,
                        leg: 0,
                        lower: false,
                    },
                    Transform::default(),
                ))
                .id();
            (app, root, limb)
        }
        let (mut host, root, limb) = app();
        host.world_mut().run_schedule(FixedUpdate);
        let start = *host.world().get::<Transform>(root).unwrap();
        for step in 1..=50 {
            host.world_mut().resource_mut::<Race>().elapsed = step as f32 * 0.25;
            host.world_mut().run_schedule(FixedUpdate);
        }
        let moving = *host.world().get::<Transform>(root).unwrap();
        let gait = *host.world().get::<Transform>(limb).unwrap();
        assert!(moving.translation.distance(start.translation) > 1.);
        host.world_mut().resource_mut::<Race>().phase = crate::game::Phase::Paused;
        for _ in 0..3 {
            host.world_mut().run_schedule(FixedUpdate);
        }
        assert_eq!(*host.world().get::<Transform>(root).unwrap(), moving);
        assert_eq!(*host.world().get::<Transform>(limb).unwrap(), gait);
        let (mut guest, remote, remote_limb) = app();
        guest.world_mut().resource_mut::<Race>().elapsed = 12.5;
        guest.world_mut().run_schedule(FixedUpdate);
        assert_eq!(*guest.world().get::<Transform>(remote).unwrap(), moving);
        assert_eq!(*guest.world().get::<Transform>(remote_limb).unwrap(), gait);
        host.world_mut().resource_mut::<Race>().elapsed = 0.;
        host.world_mut().run_schedule(FixedUpdate);
        assert_eq!(*host.world().get::<Transform>(root).unwrap(), start);
    }
}
