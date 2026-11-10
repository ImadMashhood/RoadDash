//! Approved polygonal metropolis, chunk-batched with independent static collision.
pub mod facades;
pub mod geometry;
pub mod material;
pub mod navigation;
pub mod streetscape;
pub mod survey;
use crate::{
    track::Track,
    world::{Art, Ground, Obstacle},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
#[derive(Component)]
pub struct MetroChunk;
#[derive(Resource, Default, serde::Serialize)]
pub struct Census {
    pub buildings: usize,
    pub chunks: usize,
    pub render_triangles: usize,
    pub collision_triangles: usize,
    pub source_collision_batches: usize,
    pub static_collision_batches: usize,
    pub physical_routes: usize,
    pub crossing_cars: usize,
    pub street_props: usize,
    pub parked_cars: usize,
    pub walking_pedestrians: usize,
    pub source_layout_sha256: String,
}
#[derive(Component)]
pub struct MetroWalker {
    pub station: f32,
}
#[derive(Component)]
pub struct Crossing {
    pub distance: f32,
    pub junction: Vec3,
}
#[allow(clippy::too_many_arguments)]
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    facade_materials: &mut Assets<material::FacadeMaterial>,
    server: Option<&AssetServer>,
    art: &Art,
    track: &Track,
) {
    let m = track.metropolis.as_ref().expect("metropolis content");
    if let Some(server) = server {
        commands.insert_resource(facades::Kit::load(server, track));
    }
    commands.insert_resource(navigation::Navigation::new(m));
    let mut census = geometry::spawn(
        commands,
        meshes,
        materials,
        images,
        facade_materials,
        m,
        track,
    );
    let window = art.metal.clone();
    for (i, j) in m.junctions.iter().enumerate() {
        let street = m.streets.iter().find(|s| s.id == j.street_id).unwrap();
        let a = Vec2::from_array(street.points[0]);
        let b = Vec2::from_array(*street.points.last().unwrap());
        let axis = (b - a).normalize();
        let axis = Vec3::new(axis.x, 0., axis.y);
        let p = Vec3::new(
            j.point[0],
            track.terrain_height(j.point[0], j.point[1]),
            j.point[1],
        );
        if i % 2 == 0 {
            for sign in [-1., 1.] {
                let axis = axis * sign;
                let center = p + Vec3::Y.cross(axis) * ((street.width_m * 0.5 - 1.) * 0.5);
                let pos = center - axis * 55.;
                let q = Quat::from_rotation_y(axis.x.atan2(axis.z));
                let root = commands
                    .spawn((
                        Transform::from_translation(pos).with_rotation(q),
                        Visibility::default(),
                        RigidBody::KinematicPositionBased,
                        Collider::compound(vec![(
                            Vec3::Y * 0.95,
                            Quat::IDENTITY,
                            Collider::cuboid(0.95, 0.9, 1.95),
                        )]),
                        Velocity::zero(),
                        Obstacle,
                        crate::environment::CrossTraffic,
                        ActiveEvents::COLLISION_EVENTS,
                        Ccd::enabled(),
                        crate::environment::Patrol {
                            center,
                            axis,
                            extent: 70.,
                            speed: 9.,
                            phase: 0.,
                            one_way: true,
                        },
                        Crossing {
                            distance: -55.,
                            junction: p,
                        },
                    ))
                    .id();
                if let Some(props) = &art.props {
                    let visual = props.vehicle(commands, i);
                    crate::runtime::attach_visuals(commands, root, &[visual]);
                } else {
                    let child = crate::world::part(
                        commands,
                        art.cube.clone(),
                        window.clone(),
                        Vec3::Y,
                        Vec3::new(1.9, 1.8, 3.9),
                        Quat::IDENTITY,
                    );
                    commands.entity(root).add_child(child);
                }
                census.crossing_cars += 1;
            }
        }
    }
    // Existing authored scenery/skins and their physical owners are reused.
    if art.props.is_some() {
        for (route, path) in m.paths.iter().enumerate() {
            for i in (0..path.points.len()).step_by(12) {
                let d = path.progress[i];
                if route == 1 && !(3350. ..5362.5).contains(&d) {
                    continue;
                }
                let (p, q) = track.sample_route(route, d, 0.);
                if d < 2700. && i % 36 == 0 {
                    let p = p + q * Vec3::X * 16.;
                    crate::environment::spawn_cow(
                        commands,
                        art,
                        p.with_y(track.terrain_height(p.x, p.z)),
                        q,
                    );
                }
            }
        }
    }
    if art.props.is_some() {
        for walker in &m.pedestrians {
            let p = Vec3::new(
                walker.position[0],
                track.terrain_height(walker.position[0], walker.position[1]) + 0.15,
                walker.position[1],
            );
            let entity = crate::environment::spawn_pedestrian_patrol(
                commands,
                art,
                p,
                Quat::from_rotation_y(walker.heading),
                walker.station,
                walker.extent,
            );
            commands.entity(entity).insert((
                Name::new(walker.id.clone()),
                MetroWalker {
                    station: walker.station,
                },
            ));
            census.walking_pedestrians += 1;
        }
    }
    census.street_props = streetscape::spawn(commands, art, track);
    if let Some(props) = &art.props {
        for (i, car) in m.parked_cars.iter().enumerate() {
            let p = Vec3::new(
                car.position[0],
                track.terrain_height(car.position[0], car.position[1]) + 0.025,
                car.position[1],
            );
            let root = commands
                .spawn((
                    Name::new(car.id.clone()),
                    RigidBody::Fixed,
                    Collider::compound(vec![(
                        Vec3::Y * 0.95,
                        Quat::IDENTITY,
                        Collider::cuboid(0.9, 0.9, 1.9),
                    )]),
                    Transform::from_translation(p)
                        .with_rotation(Quat::from_rotation_y(car.heading)),
                    Visibility::default(),
                    Obstacle,
                    crate::environment::CrossTraffic,
                ))
                .id();
            let visual = props.vehicle(commands, i);
            crate::runtime::attach_visuals(commands, root, &[visual]);
            census.parked_cars += 1;
        }
    }
    // Farm buildings keep original authored art rather than generic city towers.
    for (n, d) in [420., 980., 1560., 2220., 2810.].into_iter().enumerate() {
        let (p, q) = track.sample(d, if n % 2 == 0 { -65. } else { 65. });
        let p = p.with_y(track.terrain_height(p.x, p.z));
        if let Some(server) = server {
            commands.spawn((
                SceneRoot(server.load("architecture://models/BARN12_lod1.glb#Scene0")),
                Transform::from_translation(p).with_rotation(q),
                Visibility::default(),
            ));
        }
        commands.spawn((
            RigidBody::Fixed,
            Collider::cuboid(9., 4., 8.),
            Transform::from_translation(p + Vec3::Y * 4.).with_rotation(q),
            Obstacle,
        ));
    }
    for gate in &track.definition.checkpoints {
        let d = gate.fraction * track.length;
        for route in 0..if (3350. ..5362.5).contains(&d) { 2 } else { 1 } {
            let (p, q) = track.sample_route(route, d, 0.);
            for side in [-1., 1.] {
                let e = crate::world::part(
                    commands,
                    art.cube.clone(),
                    art.white.clone(),
                    p + q * Vec3::X * side * (track.half_width(d) + 1.) + Vec3::Y * 2.,
                    Vec3::new(0.15, 4., 0.15),
                    q,
                );
                commands.entity(e).insert((
                    RigidBody::Fixed,
                    Collider::cuboid(0.5, 0.5, 0.5),
                    Obstacle,
                ));
            }
        }
    }
    commands.insert_resource(crate::environment::TrackInspector {
        visible: false,
        render_triangles: census.render_triangles,
        collision_triangles: census.collision_triangles,
    });
    info!(
        "Approved metropolis: {} buildings / {} chunks / {} triangles / two routes",
        census.buildings, census.chunks, census.render_triangles
    );
    commands.insert_resource(census);
}
pub fn crossing_traffic(
    time: Res<Time<Fixed>>,
    race: Res<crate::game::Race>,
    track: Res<Track>,
    riders: Query<(&crate::game::Rider, &Transform), Without<crate::environment::Patrol>>,
    main_cars: Query<
        &Transform,
        (
            With<crate::game::Traffic>,
            Without<crate::environment::Patrol>,
        ),
    >,
    mut cars: Query<(
        &crate::environment::Patrol,
        &mut Crossing,
        &mut Transform,
        &mut Velocity,
    )>,
) {
    if race.phase != crate::game::Phase::Racing {
        return;
    }
    let dt = time.delta_secs();
    for (path, mut cross, mut pose, mut velocity) in &mut cars {
        let wait = riders
            .iter()
            .any(|(r, t)| r.speed > 1. && t.translation.xz().distance(cross.junction.xz()) < 45.)
            || main_cars
                .iter()
                .any(|t| t.translation.xz().distance(cross.junction.xz()) < 60.);
        let next = cross.distance + path.speed * dt;
        if !(wait && cross.distance <= -12. && next > -12.) {
            cross.distance = next;
        }
        if cross.distance > 70. {
            cross.distance = -70.;
        }
        let mut p = path.center + path.axis * cross.distance;
        p.y = track.terrain_height(p.x, p.z) + 0.025;
        velocity.linvel = if dt > 0. && pose.translation.distance(p) < 5. {
            (p - pose.translation) / dt
        } else {
            Vec3::ZERO
        };
        pose.translation = p;
    }
}
