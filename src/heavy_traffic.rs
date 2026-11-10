//! Hills-only original working road traffic. Articulated trailers remain in the
//! owning Traffic collider, so impacts, forecasts and snapshots share identity.
use crate::{
    game::Traffic,
    track::Track,
    world::{self, Art},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
pub use rr_core::traffic::HeavyKind as Kind;
#[derive(Component)]
pub struct HeavyTraffic {
    pub kind: Kind,
    pub trailer: Option<Entity>,
    last_trailer: Transform,
    station: f32,
}
#[derive(Component)]
pub struct HeavyTrailer;
#[derive(Component)]
pub struct HeavyTire;
pub const SEMIS: usize = 8;
pub const TRACTORS: usize = 6;
pub fn selection(index: usize) -> Option<(Kind, f32)> {
    const TRUCKS: [f32; SEMIS] = [450., 1350., 2400., 4200., 5900., 6900., 8100., 10300.];
    const FARMS: [f32; TRACTORS] = [1950., 2750., 3500., 5100., 5750., 7900.];
    if (24..24 + SEMIS).contains(&index) {
        Some((Kind::Semi, TRUCKS[index - 24]))
    } else if (24 + SEMIS..24 + SEMIS + TRACTORS).contains(&index) {
        Some((Kind::Tractor, FARMS[index - 24 - SEMIS]))
    } else {
        None
    }
}
fn block(
    commands: &mut Commands,
    art: &Art,
    p: Vec3,
    size: Vec3,
    material: Handle<StandardMaterial>,
) -> Entity {
    world::part(
        commands,
        art.cube.clone(),
        material,
        p,
        size,
        Quat::IDENTITY,
    )
}
fn wheel(commands: &mut Commands, art: &Art, p: Vec3, radius: f32, width: f32) -> Entity {
    let tire = world::part(
        commands,
        art.wheel.clone(),
        art.dark.clone(),
        p,
        Vec3::new(radius / 0.4, width / 0.2, radius / 0.4),
        Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
    );
    commands.entity(tire).insert(HeavyTire);
    // Separate inset steel hub; the actual tire count stays inspectable.
    let hub = world::part(
        commands,
        art.wheel.clone(),
        art.metal.clone(),
        p,
        Vec3::new(
            radius * 0.5 / 0.4,
            (width + 0.015) / 0.2,
            radius * 0.5 / 0.4,
        ),
        Quat::from_rotation_z(std::f32::consts::FRAC_PI_2),
    );
    commands.entity(tire).add_child(hub);
    // Hub uses the tire's scaled coordinates; restore local physical sizing.
    commands
        .entity(hub)
        .insert(Transform::from_scale(Vec3::new(0.5, 1.04, 0.5)));
    tire
}
pub fn spawn(commands: &mut Commands, art: &Art, owner: Entity, kind: Kind, index: usize) {
    let paint = art.colors[(index + 3) % art.colors.len()].clone();
    let mut pieces = vec![];
    let mut add = |p, size, mat| pieces.push(block(commands, art, p, size, mat));
    match kind {
        Kind::Semi => {
            add(
                Vec3::new(0., 0.65, -0.1),
                Vec3::new(2.45, 0.38, 6.4),
                art.dark.clone(),
            );
            add(
                Vec3::new(0., 1.3, 2.05),
                Vec3::new(2.35, 1.25, 2.4),
                paint.clone(),
            );
            add(
                Vec3::new(0., 2.1, 0.05),
                Vec3::new(2.4, 2.25, 2.3),
                paint.clone(),
            );
            add(
                Vec3::new(0., 3.35, -0.12),
                Vec3::new(2.43, 0.3, 2.6),
                paint.clone(),
            );
            add(
                Vec3::new(0., 2.65, 1.22),
                Vec3::new(2.1, 0.8, 0.035),
                art.dark.clone(),
            );
            add(
                Vec3::new(0., 1.2, 3.29),
                Vec3::new(1.4, 0.85, 0.08),
                art.metal.clone(),
            );
            add(
                Vec3::new(0., 0.55, 3.35),
                Vec3::new(2.6, 0.25, 0.18),
                art.metal.clone(),
            );
            for x in [-1., 1.] {
                add(
                    Vec3::new(x * 1.23, 2.65, 0.25),
                    Vec3::new(0.04, 0.8, 1.4),
                    art.dark.clone(),
                );
                add(
                    Vec3::new(x * 1.38, 2.4, 0.95),
                    Vec3::new(0.18, 0.4, 0.26),
                    art.metal.clone(),
                );
                add(
                    Vec3::new(x * 1.05, 2.65, -1.25),
                    Vec3::new(0.14, 2.3, 0.14),
                    art.metal.clone(),
                );
                add(
                    Vec3::new(x * 0.98, 1.2, 3.34),
                    Vec3::new(0.36, 0.3, 0.045),
                    art.white.clone(),
                );
                add(
                    Vec3::new(x * 1.2, 0.8, -0.65),
                    Vec3::new(0.2, 0.38, 1.1),
                    art.metal.clone(),
                );
            }
            for x in [-1.16, 1.16] {
                pieces.push(wheel(commands, art, Vec3::new(x, 0.53, 2.1), 0.53, 0.25));
            }
            for z in [-0.8, -2.] {
                for x in [-1.17, -0.92, 0.92, 1.17] {
                    pieces.push(wheel(commands, art, Vec3::new(x, 0.53, z), 0.53, 0.23));
                }
            }
            let trailer = commands
                .spawn((
                    HeavyTrailer,
                    Transform::from_xyz(0., 0., -7.5),
                    Visibility::default(),
                ))
                .id();
            let mut cargo = vec![
                block(
                    commands,
                    art,
                    Vec3::new(0., 2.25, 0.),
                    Vec3::new(2.58, 2.7, 11.8),
                    art.white.clone(),
                ),
                block(
                    commands,
                    art,
                    Vec3::new(0., 0.7, 0.),
                    Vec3::new(2.5, 0.3, 11.8),
                    art.dark.clone(),
                ),
            ];
            for side in [-1., 1.] {
                cargo.push(block(
                    commands,
                    art,
                    Vec3::new(side * 1.3, 1.2, 0.),
                    Vec3::new(0.035, 0.25, 11.6),
                    paint.clone(),
                ));
                for rib in 0..20 {
                    cargo.push(block(
                        commands,
                        art,
                        Vec3::new(side * 1.301, 2.45, -5.6 + rib as f32 * 0.59),
                        Vec3::new(0.045, 2.15, 0.025),
                        art.metal.clone(),
                    ));
                }
                cargo.push(block(
                    commands,
                    art,
                    Vec3::new(side * 0.66, 2.2, -5.92),
                    Vec3::new(1.24, 2.6, 0.045),
                    art.metal.clone(),
                ));
                cargo.push(block(
                    commands,
                    art,
                    Vec3::new(side * 1.05, 0.75, -5.95),
                    Vec3::new(0.3, 0.18, 0.045),
                    art.colors[13].clone(),
                ));
            }
            for z in [-3.9, -5.2] {
                for x in [-1.17, -0.92, 0.92, 1.17] {
                    cargo.push(wheel(commands, art, Vec3::new(x, 0.53, z), 0.53, 0.23));
                }
            }
            commands.entity(trailer).add_children(&cargo);
            pieces.push(trailer);
            commands.entity(owner).insert(HeavyTraffic {
                kind,
                trailer: Some(trailer),
                last_trailer: Transform::from_xyz(0., 0., -7.5),
                station: selection(index).unwrap().1,
            });
        }
        Kind::Tractor => {
            add(
                Vec3::new(0., 0.8, 0.),
                Vec3::new(1.55, 0.4, 4.5),
                art.dark.clone(),
            );
            add(
                Vec3::new(0., 1.45, 1.0),
                Vec3::new(1.45, 1.2, 2.6),
                paint.clone(),
            );
            add(
                Vec3::new(0., 2.25, -0.85),
                Vec3::new(1.6, 1.8, 1.75),
                art.dark.clone(),
            );
            add(
                Vec3::new(0., 3.21, -0.85),
                Vec3::new(2.05, 0.2, 2.15),
                paint.clone(),
            );
            add(
                Vec3::new(0., 1.35, 2.33),
                Vec3::new(1.25, 0.8, 0.045),
                art.metal.clone(),
            );
            add(
                Vec3::new(0.55, 2.6, 0.85),
                Vec3::new(0.13, 2.1, 0.13),
                art.dark.clone(),
            );
            for x in [-1., 1.] {
                add(
                    Vec3::new(x * 1.12, 1.86, -1.25),
                    Vec3::new(0.6, 0.15, 1.9),
                    paint.clone(),
                );
                add(
                    Vec3::new(x * 0.8, 2.25, -0.85),
                    Vec3::new(0.1, 1.7, 0.1),
                    paint.clone(),
                );
                add(
                    Vec3::new(x * 0.5, 1.65, 2.36),
                    Vec3::new(0.25, 0.22, 0.04),
                    art.white.clone(),
                );
            }
            for x in [-1.14, 1.14] {
                pieces.push(wheel(commands, art, Vec3::new(x, 0.92, -1.25), 0.92, 0.48));
            }
            for x in [-0.95, 0.95] {
                pieces.push(wheel(commands, art, Vec3::new(x, 0.51, 1.65), 0.51, 0.32));
            }
            commands.entity(owner).insert(HeavyTraffic {
                kind,
                trailer: None,
                last_trailer: Transform::IDENTITY,
                station: selection(index).unwrap().1,
            });
        }
    }
    commands
        .entity(owner)
        .insert(collider(kind, Transform::from_xyz(0., 0., -7.5)));
    crate::runtime::attach_visuals(commands, owner, &pieces);
}
fn basis(forward: Vec3) -> Quat {
    let z = forward.normalize();
    let x = Vec3::Y.cross(z).normalize();
    let y = z.cross(x);
    Quat::from_mat3(&Mat3::from_cols(x, y, z))
}
/// Trailer hitch stays attached to the tractor; its rear axles follow the same
/// route and lane, including the town split and merge. Uses replicated root pose.
pub fn trailer_pose(track: &Track, car: &Traffic, root: Transform) -> Transform {
    trailer_at(track, car, root, car.progress)
}
fn trailer_at(track: &Track, car: &Traffic, root: Transform, station: f32) -> Transform {
    let lane = track.lane_at(car.route, station, root.translation);
    let road = track.sample_route(car.route, station, lane).1 * Vec3::Z;
    let direction = if (root.rotation * Vec3::Z).dot(road) >= 0. {
        1.
    } else {
        -1.
    };
    let hitch = root.translation - root.rotation * Vec3::Z * 2.8;
    let rear = track
        .street_sample(car.route, station - direction * 11.6, lane)
        .0;
    let q = basis(hitch - rear);
    let world = Transform::from_translation(hitch - q * Vec3::Z * 4.7).with_rotation(q);
    Transform::from_translation(root.rotation.inverse() * (world.translation - root.translation))
        .with_rotation(root.rotation.inverse() * world.rotation)
}
fn collider(kind: Kind, trailer: Transform) -> Collider {
    let mut boxes = match kind {
        Kind::Semi => vec![
            (
                Vec3::new(0., 1.8, 0.65),
                Quat::IDENTITY,
                Collider::cuboid(1.48, 1.75, 2.85),
            ),
            (
                Vec3::new(0., 0.55, -1.95),
                Quat::IDENTITY,
                Collider::cuboid(1.31, 0.5, 1.35),
            ),
        ],
        Kind::Tractor => vec![
            (
                Vec3::new(0., 1.75, -0.7),
                Quat::IDENTITY,
                Collider::cuboid(0.99, 1.55, 1.2),
            ),
            (
                Vec3::new(0., 1.2, 1.15),
                Quat::IDENTITY,
                Collider::cuboid(0.85, 0.85, 1.23),
            ),
            (
                Vec3::new(0., 0.92, -1.25),
                Quat::IDENTITY,
                Collider::cuboid(1.42, 0.92, 0.92),
            ),
            (
                Vec3::new(0., 0.51, 1.65),
                Quat::IDENTITY,
                Collider::cuboid(1.14, 0.51, 0.51),
            ),
        ],
    };
    if kind == Kind::Semi {
        for (p, half) in [
            (Vec3::new(0., 2.25, 0.), Vec3::new(1.31, 1.35, 5.92)),
            (Vec3::new(0., 0.7, 0.), Vec3::new(1.28, 0.15, 5.9)),
            (Vec3::new(0., 0.53, -4.55), Vec3::new(1.3, 0.53, 1.2)),
        ] {
            boxes.push((
                trailer.transform_point(p),
                trailer.rotation,
                Collider::cuboid(half.x, half.y, half.z),
            ));
        }
    }
    Collider::compound(boxes)
}
/// Fixed before Rapier SyncBackend, on hosts and guests (root is replicated).
/// No assets/meshes spawned here; only bounded transforms and compound shapes.
pub fn update(
    track: Res<Track>,
    mut roots: Query<(&Traffic, &Transform, &mut Collider, &mut HeavyTraffic)>,
    mut trailers: Query<&mut Transform, (With<HeavyTrailer>, Without<Traffic>)>,
) {
    if !track.is_hills() {
        return;
    }
    for (car, root, mut shape, mut heavy) in &mut roots {
        if heavy.kind != Kind::Semi {
            continue;
        }
        // Hosts already have authoritative progress. Guests use three bounded
        // local projections; a full search occurs only after a remote recycle/join.
        let authored = track.street_sample(car.route, car.progress, car.lane).0;
        let mut station = if authored.distance_squared(root.translation) < 1. {
            car.progress
        } else {
            heavy.station
        };
        if authored.distance_squared(root.translation) >= 1. {
            for _ in 0..3 {
                let (p, q) = track.sample_route(car.route, station, 0.);
                station += (root.translation - p).dot(q * Vec3::Z).clamp(-20., 20.);
            }
            if track
                .street_sample(car.route, station, car.lane)
                .0
                .distance_squared(root.translation)
                > 900.
            {
                station = track.project_route(car.route, root.translation, station).0;
            }
        }
        heavy.station = station;
        let local = trailer_at(&track, car, *root, station);
        if let Some(entity) = heavy.trailer
            && let Ok(mut pose) = trailers.get_mut(entity)
        {
            *pose = local;
        }
        // Ignore sub-centimetre floating-point jitter on straight spans.
        if local
            .translation
            .distance_squared(heavy.last_trailer.translation)
            > 0.000025
            || local.rotation.angle_between(heavy.last_trailer.rotation) > 0.0005
        {
            *shape = collider(heavy.kind, local);
            heavy.last_trailer = local;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    #[test]
    fn full_truck_and_tractor_geometry_stays_in_the_carriageway_on_both_routes() {
        let track = Track::hills().unwrap();
        let mut largest_lane = 0f32;
        let mut wheel_error = 0f32;
        let mut articulation = 0f32;
        for route in 0..2 {
            for direction in [-1., 1.] {
                for station in (35..12465).step_by(5) {
                    let station = station as f32;
                    let car = Traffic {
                        route,
                        progress: station,
                        lane: direction * 3.1,
                        speed: direction * 12.,
                    };
                    let (p, q) = track.street_sample(route, station, car.lane);
                    let root = Transform::from_translation(p).with_rotation(
                        q * if direction < 0. {
                            Quat::from_rotation_y(std::f32::consts::PI)
                        } else {
                            Quat::IDENTITY
                        },
                    );
                    let trailer = trailer_pose(&track, &car, root);
                    articulation = articulation.max(trailer.rotation.angle_between(Quat::IDENTITY));
                    for (part, length, width) in
                        [(Transform::IDENTITY, 3.4, 1.48), (trailer, 5.92, 1.31)]
                    {
                        for x in [-width, width] {
                            for z in [-length, length] {
                                let corner =
                                    root.transform_point(part.transform_point(Vec3::new(x, 0., z)));
                                let (s, lane, _) = track.project_route(route, corner, station);
                                largest_lane = largest_lane.max(lane.abs());
                                assert!(
                                    lane * direction > 0.1,
                                    "truck enters oncoming lane at {station}: lane={lane}"
                                );
                                assert!(
                                    lane.abs() < track.half_width(s) - 0.05,
                                    "truck corner leaves road route={route} station={station} direction={direction} lane={lane}"
                                );
                            }
                        }
                    }
                    for x in [-1.17, 1.17] {
                        for z in [-3.9, -5.2] {
                            let wheel =
                                root.transform_point(trailer.transform_point(Vec3::new(x, 0., z)));
                            let (s, lane, _) = track.project_route(route, wheel, station);
                            let road = track.sample_route(route, s, lane).0;
                            wheel_error = wheel_error.max((road.y - wheel.y).abs());
                        }
                    }
                }
            }
        }
        eprintln!(
            "Heavy traffic route sweep: max lane={largest_lane}, trailer wheel support error={wheel_error}, max articulation={articulation}"
        );
        assert!(
            wheel_error < 0.2,
            "trailer cannot visibly bridge unsupported road relief: {wheel_error}"
        );
        assert!(
            articulation > 0.02,
            "trailer must actually articulate on curves"
        );
    }
    #[test]
    fn semis_have_eighteen_tires_and_large_articulated_root_collision() {
        let mut app = crate::game::tests::simulation();
        app.insert_resource(Track::hills().unwrap());
        let (semi, tractor) = app
            .world_mut()
            .run_system_once(|mut commands: Commands, art: Res<Art>| {
                let semi = commands
                    .spawn((Transform::default(), Visibility::default()))
                    .id();
                spawn(&mut commands, &art, semi, Kind::Semi, 24);
                let tractor = commands
                    .spawn((Transform::default(), Visibility::default()))
                    .id();
                spawn(&mut commands, &art, tractor, Kind::Tractor, 32);
                (semi, tractor)
            })
            .unwrap();
        fn tires(world: &World, entity: Entity) -> usize {
            usize::from(world.get::<HeavyTire>(entity).is_some())
                + world.get::<Children>(entity).map_or(0, |children| {
                    children.iter().map(|c| tires(world, c)).sum::<usize>()
                })
        }
        assert_eq!(tires(app.world(), semi), 18);
        assert_eq!(tires(app.world(), tractor), 4);
        let (lo, hi) = crate::recovery::collider_bounds(
            &Transform::IDENTITY,
            app.world().get::<Collider>(semi).unwrap(),
        );
        assert!(hi.z - lo.z > 15. && hi.y > 3.5 && hi.x - lo.x > 2.8);
        let (lo, hi) = crate::recovery::collider_bounds(
            &Transform::IDENTITY,
            app.world().get::<Collider>(tractor).unwrap(),
        );
        assert!(hi.z - lo.z > 4.5 && hi.y > 3. && hi.x - lo.x > 2.7);
        assert!(
            app.world()
                .get::<HeavyTraffic>(semi)
                .unwrap()
                .trailer
                .is_some()
        );
    }
    #[test]
    fn replicated_root_updates_trailer_and_full_collision_without_spawning_assets() {
        let mut app = crate::game::tests::simulation();
        app.insert_resource(Track::hills().unwrap());
        let root = app
            .world_mut()
            .run_system_once(|mut commands: Commands, art: Res<Art>, track: Res<Track>| {
                let car = Traffic {
                    route: 1,
                    progress: 6200.,
                    lane: 3.1,
                    speed: 12.,
                };
                let (p, q) = track.street_sample(1, 6200., 3.1);
                let root = commands
                    .spawn((
                        car,
                        Transform::from_translation(p).with_rotation(q),
                        Visibility::default(),
                    ))
                    .id();
                spawn(&mut commands, &art, root, Kind::Semi, 24);
                root
            })
            .unwrap();
        app.world_mut().run_system_once(update).unwrap();
        let count = app.world().entities().len();
        let trailer = app
            .world()
            .get::<HeavyTraffic>(root)
            .unwrap()
            .trailer
            .unwrap();
        for station in [6250., 6251., 7395., 7405.] {
            let (p, q) = app
                .world()
                .resource::<Track>()
                .street_sample(1, station, 3.1);
            // Replication supplies root transform, not a separate trailer packet.
            *app.world_mut().get_mut::<Transform>(root).unwrap() =
                Transform::from_translation(p).with_rotation(q);
            app.world_mut().run_system_once(update).unwrap();
            let car = Traffic {
                route: 1,
                progress: station,
                lane: 3.1,
                speed: 12.,
            };
            let expected = trailer_pose(
                app.world().resource::<Track>(),
                &car,
                *app.world().get::<Transform>(root).unwrap(),
            );
            let actual = app.world().get::<Transform>(trailer).unwrap();
            assert!(actual.translation.distance(expected.translation) < 0.03);
            assert!(actual.rotation.angle_between(expected.rotation) < 0.003);
            let (lo, _) = crate::recovery::collider_bounds(
                &Transform::IDENTITY,
                app.world().get::<Collider>(root).unwrap(),
            );
            assert!(lo.z < -13.);
            assert_eq!(app.world().entities().len(), count);
        }
        let before = *app.world().get::<Transform>(trailer).unwrap();
        for _ in 0..8 {
            app.world_mut().run_system_once(update).unwrap();
        }
        assert_eq!(*app.world().get::<Transform>(trailer).unwrap(), before);
    }
    #[test]
    fn hitch_stays_connected_and_direction_is_inferred_from_replicated_pose() {
        let track = Track::hills().unwrap();
        for direction in [-1., 1.] {
            let car = Traffic {
                route: 1,
                progress: 6200.,
                lane: direction * 3.1,
                speed: 0.,
            };
            let (p, q) = track.street_sample(car.route, car.progress, car.lane);
            let root = Transform::from_translation(p).with_rotation(
                q * if direction < 0. {
                    Quat::from_rotation_y(std::f32::consts::PI)
                } else {
                    Quat::IDENTITY
                },
            );
            let trailer = trailer_pose(&track, &car, root);
            assert!(
                root.transform_point(trailer.transform_point(Vec3::Z * 4.7))
                    .distance(root.transform_point(Vec3::NEG_Z * 2.8))
                    < 0.0005
            );
            assert!(
                (root.rotation * trailer.rotation * Vec3::Z).dot(root.rotation * Vec3::Z) > 0.9
            );
        }
    }
}
