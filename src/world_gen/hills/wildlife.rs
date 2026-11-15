//! Scenic off-road herds and solid Hills cattle crossings.
use super::{Track, props::Model};
use crate::world::Art;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rr_core::wildlife::{self as rules, Species};
const SITES: [(usize, f32, f32); rules::HERD_COUNT] = [
    (0, 2050., 75.),
    (0, 3100., -75.),
    (0, 5320., 75.),
    (0, 8700., -75.),
    (0, 9600., 80.),
    (0, 10800., -80.),
];
const SAMPLES: usize = 128;
const CROSSING_STATIONS: [f32; 2] = [8870., 11250.];
const CROSSING_ROWS: usize = 4;
const CROSSING_COLUMNS: usize = 5;
const CROSSING_MEMBERS: usize = CROSSING_ROWS * CROSSING_COLUMNS;
const CROSSING_SPEED: f32 = 0.88;
const CROSSING_EXTENT: f32 = 20.;
const ROW_ACROSS: [f32; CROSSING_ROWS] = [-9.6, -3.6, 2.7, 9.5];
const ROW_STAGGER: [f32; CROSSING_ROWS] = [-1.9, 1.8, -1.0, 2.25];
const COLUMN_ALONG: [f32; CROSSING_COLUMNS] = [-13.2, -6.6, 0.4, 7.5, 14.1];
const COLUMN_SKEW: [f32; CROSSING_COLUMNS] = [-0.3, 0.15, 0.45, -0.2, 0.3];
const CROSSING_RESERVED_ACROSS: f32 = CROSSING_EXTENT + 11.2;
const CROSSING_RESERVED_ALONG: f32 = 32.;
const SUPPORT_INTERVALS: usize = 160;
pub const NEAR: f32 = 100.;
pub const FAR: f32 = 500.;
#[derive(Component)]
pub struct WildAnimal {
    pub herd: usize,
    pub member: usize,
}
#[derive(Component)]
pub struct CrossingCow;
/// Support at 0.25 m intervals along the fixed crossing path. Building this
/// once keeps terrain and road projection out of the 40-cow fixed-tick loop.
#[derive(Component)]
pub struct CrossingGround([f32; SUPPORT_INTERVALS + 1]);
impl CrossingGround {
    fn from_path(track: &Track, station: f32, center: Vec3, axis: Vec3) -> Self {
        Self(std::array::from_fn(|i| {
            let offset = (2. * i as f32 / SUPPORT_INTERVALS as f32 - 1.) * CROSSING_EXTENT;
            super::geometry::crossing_support(track, 0, center + axis * offset, station)
        }))
    }
    pub(crate) fn at(&self, fraction: f32) -> f32 {
        let distance = (fraction.clamp(-1., 1.) + 1.) * 0.5 * SUPPORT_INTERVALS as f32;
        let i = (distance as usize).min(SUPPORT_INTERVALS - 1);
        self.0[i].lerp(self.0[i + 1], distance - i as f32)
    }
}

#[derive(Resource, Default)]
pub struct Wildlife {
    pub herds: Vec<Herd>,
    pub active_animals: usize,
    pub animated_limbs: usize,
    pub visible_meshes: usize,
    pub update_micros: f64,
}
pub struct Herd {
    pub center: Vec3,
    animals: Vec<Animal>,
}
struct Animal {
    root: Entity,
    near: Entity,
    far: Entity,
    legs: [Entity; 4],
    path: Vec<Vec3>,
    visible: bool,
    near_lod: bool,
}
pub fn center(track: &Track, herd: usize) -> (Vec3, Quat) {
    let (route, station, lane) = SITES[herd];
    let (p, q) = track.sample_route(route, station, 0.);
    let f = (q * Vec3::Z).with_y(0.).normalize();
    (
        p - q * Vec3::X * lane,
        Quat::from_rotation_y(f.x.atan2(f.z)),
    )
}
pub(super) fn reserved(track: &Track, point: Vec3, radius: f32) -> bool {
    (0..rules::HERD_COUNT).any(|h| {
        let (p, q) = center(track, h);
        let l = q.inverse() * (point - p);
        l.x.abs() < rules::HALF_WIDTH + radius && l.z.abs() < rules::HALF_LENGTH + radius
    }) || CROSSING_STATIONS.iter().any(|&station| {
        let (p, q) = track.sample_route(0, station, 0.);
        let across = (q * Vec3::X).with_y(0.).normalize();
        let delta = point - p;
        delta.dot(across).abs() < CROSSING_RESERVED_ACROSS + radius
            && delta.dot(across.cross(Vec3::Y)).abs() < CROSSING_RESERVED_ALONG + radius
    })
}
/// Solid cattle cross in staggered rows with space for a mounted racer.
pub fn spawn_crossing_cattle(commands: &mut Commands, art: &Art, track: &Track) {
    if std::env::var("ROADDASH_WILDLIFE").as_deref() == Ok("0") {
        return;
    }
    for (group, &station) in CROSSING_STATIONS.iter().enumerate() {
        for member in 0..CROSSING_MEMBERS {
            let (center, axis, phase) = cattle_path(track, station, member);
            let ground = CrossingGround::from_path(track, station, center, axis);
            let start = center + axis * ((phase - 1.) * CROSSING_EXTENT);
            let p = start.with_y(ground.at(phase - 1.) + 0.015);
            let entity = crate::environment::spawn_cow(commands, art, p, cattle_heading(axis));
            commands.entity(entity).insert((
                Name::new(format!(
                    "Hills crossing cattle herd {group} member {member}"
                )),
                CrossingCow,
                ground,
                RigidBody::KinematicPositionBased,
                ActiveCollisionTypes::all(),
                crate::world::Obstacle,
                crate::environment::Patrol {
                    center,
                    axis,
                    extent: CROSSING_EXTENT,
                    speed: CROSSING_SPEED,
                    phase,
                    one_way: false,
                },
            ));
        }
    }
}
fn cattle_path(track: &Track, station: f32, member: usize) -> (Vec3, Vec3, f32) {
    let (point, frame) = track.sample_route(0, station, 0.);
    let axis = (frame * Vec3::X).with_y(0.).normalize();
    let road = axis.cross(Vec3::Y);
    let row = member / CROSSING_COLUMNS;
    let column = member % CROSSING_COLUMNS;
    let along_road = COLUMN_ALONG[column] + ROW_STAGGER[row];
    let row_sign = if row.is_multiple_of(2) { 1. } else { -1. };
    let along_crossing = ROW_ACROSS[row] + row_sign * COLUMN_SKEW[column];
    (
        point + road * along_road + axis * along_crossing,
        axis,
        if station == CROSSING_STATIONS[0] {
            0.74
        } else {
            0.94
        },
    )
}

pub(crate) fn cattle_heading(direction: Vec3) -> Quat {
    Quat::from_rotation_y((-direction.z).atan2(direction.x))
}

pub(crate) fn ground_point(track: &Track, h: usize, m: usize, a: f32) -> Vec3 {
    let (p, q) = center(track, h);
    let [x, z] = rules::local(h, m, a);
    let mut p = p + q * Vec3::new(x, 0., z);
    p.y = super::geometry::ground_height(track, p.x, p.z);
    p
}
fn skin(s: Species, variant: usize) -> [f32; 3] {
    match s {
        Species::Horse => [[0.46, 0.22, 0.10], [0.72, 0.56, 0.36], [0.21, 0.16, 0.13]][variant % 3],
        Species::Bison => {
            [[0.24, 0.15, 0.09], [0.31, 0.21, 0.13], [0.19, 0.12, 0.075]][variant % 3]
        }
    }
}
fn body(s: Species, v: usize, low: bool) -> Mesh {
    let mut m = Model::default();
    let hide = skin(s, v);
    let dark = [0.075, 0.048, 0.03];
    let b = s == Species::Bison;
    m.ellipsoid(
        Vec3::new(0., 1.25, 0.),
        Vec3::new(if b { 0.64 } else { 0.43 }, 0.49, 1.02),
        hide,
        0.,
    );
    if b {
        m.ellipsoid(
            Vec3::new(0., 1.60, 0.48),
            Vec3::new(0.64, 0.66, 0.64),
            hide,
            0.,
        );
        m.ellipsoid(
            Vec3::new(0., 1.15, 1.05),
            Vec3::new(0.48, 0.53, 0.48),
            dark,
            0.,
        );
        m.ellipsoid(
            Vec3::new(0., 0.71, 1.10),
            Vec3::new(0.23, 0.35, 0.22),
            dark,
            0.,
        );
        for side in [-1., 1.] {
            m.rod(
                Vec3::new(side * 0.35, 1.55, 1.05),
                Vec3::new(side * 0.65, 1.68, 1.03),
                0.09,
                [0.72, 0.67, 0.51],
            );
            m.rod(
                Vec3::new(side * 0.65, 1.68, 1.03),
                Vec3::new(side * 0.62, 1.89, 1.01),
                0.055,
                [0.72, 0.67, 0.51],
            );
        }
    } else {
        m.rod(
            Vec3::new(0., 1.4, 0.56),
            Vec3::new(0., 2.03, 0.91),
            0.26,
            hide,
        );
        m.ellipsoid(
            Vec3::new(0., 2.04, 1.13),
            Vec3::new(0.23, 0.24, 0.47),
            hide,
            0.,
        );
        m.ellipsoid(
            Vec3::new(0., 1.92, 1.48),
            Vec3::new(0.23, 0.17, 0.18),
            dark,
            0.,
        );
        for side in [-1., 1.] {
            m.rod(
                Vec3::new(side * 0.15, 2.2, 0.87),
                Vec3::new(side * 0.18, 2.51, 0.82),
                0.065,
                hide,
            );
        }
        m.rod(
            Vec3::new(0., 2.03, 0.68),
            Vec3::new(0., 1.48, 0.32),
            0.105,
            dark,
        );
    }
    if !low {
        for side in [-1., 1.] {
            m.ellipsoid(
                Vec3::new(
                    side * if b { 0.38 } else { 0.22 },
                    if b { 1.37 } else { 2.1 },
                    if b { 1.30 } else { 1.17 },
                ),
                Vec3::splat(0.035),
                [0.015, 0.012, 0.01],
                0.,
            );
        }
    }
    m.rod(
        Vec3::new(0., 1.43, -0.93),
        Vec3::new(0., 0.50, -1.25),
        if b { 0.045 } else { 0.10 },
        dark,
    );
    if low {
        for leg in 0..4 {
            let p = hip(s, leg);
            m.rod(p, p - Vec3::Y * 0.85, 0.08, hide);
        }
    }
    m.mesh()
}
fn hip(s: Species, leg: usize) -> Vec3 {
    Vec3::new(
        if leg.is_multiple_of(2) { -1. } else { 1. }
            * if s == Species::Bison { 0.39 } else { 0.29 },
        0.92,
        if leg < 2 { 0.65 } else { -0.65 },
    )
}
fn leg(s: Species, v: usize) -> Mesh {
    let mut m = Model::default();
    let hide = skin(s, v);
    m.rod(Vec3::ZERO, Vec3::new(0., -0.42, -0.03), 0.09, hide);
    m.rod(
        Vec3::new(0., -0.42, -0.03),
        Vec3::new(0., -0.80, 0.02),
        0.065,
        hide,
    );
    m.ellipsoid(
        Vec3::new(0., -0.84, 0.06),
        Vec3::new(0.11, 0.075, 0.16),
        [0.065, 0.045, 0.03],
        0.,
    );
    m.mesh()
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
) {
    let mut state = Wildlife::default();
    if std::env::var("ROADDASH_WILDLIFE").as_deref() == Ok("0") {
        commands.insert_resource(state);
        return;
    }
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.95,
        ..default()
    });
    let assets: Vec<_> = [Species::Bison, Species::Horse]
        .into_iter()
        .map(|s| {
            (0..3)
                .map(|v| {
                    (
                        meshes.add(body(s, v, false)),
                        meshes.add(body(s, v, true)),
                        meshes.add(leg(s, v)),
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect();
    for (h, &count) in rules::COUNTS.iter().enumerate() {
        let s = rules::species(h);
        let mut herd = Herd {
            center: center(track, h).0,
            animals: Vec::new(),
        };
        for member in 0..count {
            let a = &assets[usize::from(s == Species::Horse)][member % 3];
            let path: Vec<Vec3> = (0..SAMPLES)
                .map(|i| {
                    ground_point(
                        track,
                        h,
                        member,
                        i as f32 / SAMPLES as f32 * std::f32::consts::TAU,
                    )
                })
                .collect();
            rr_content::wildlife::validate_path(
                &path.iter().map(|p| p.to_array()).collect::<Vec<_>>(),
                &path
                    .iter()
                    .map(|p| track.scenery_fits(*p, 2., 8.))
                    .collect::<Vec<_>>(),
            )
            .expect("authored wildlife path must be supported and road-clear");
            let root = commands
                .spawn((
                    WildAnimal { herd: h, member },
                    Name::new(format!("Wild {s:?} herd{h} member{member}")),
                    Transform::default(),
                    Visibility::Hidden,
                ))
                .id();
            let near = commands
                .spawn((
                    Mesh3d(a.0.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::default(),
                ))
                .id();
            let far = commands
                .spawn((
                    Mesh3d(a.1.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::default(),
                    Visibility::Hidden,
                ))
                .id();
            let legs = std::array::from_fn(|l| {
                commands
                    .spawn((
                        Mesh3d(a.2.clone()),
                        MeshMaterial3d(material.clone()),
                        Transform::from_translation(hip(s, l)),
                    ))
                    .id()
            });
            commands.entity(near).add_children(&legs);
            commands.entity(root).add_children(&[near, far]);
            herd.animals.push(Animal {
                root,
                near,
                far,
                legs,
                path,
                visible: false,
                near_lod: true,
            });
        }
        state.herds.push(herd);
    }
    commands.insert_resource(state);
}
/// Rendering only, after the final chase-camera update and before propagation.
/// Hidden herds do one center-distance test; no member paths, bones or terrain.
pub fn animate(
    race: Res<crate::game::Race>,
    mut state: ResMut<Wildlife>,
    camera: Query<&Transform, With<crate::game::ChaseCamera>>,
    mut transforms: Query<&mut Transform, Without<crate::game::ChaseCamera>>,
    mut visibility: Query<&mut Visibility>,
) {
    let start = std::time::Instant::now();
    let Ok(camera) = camera.single() else {
        return;
    };
    let nearest = state
        .herds
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            camera
                .translation
                .distance_squared(a.center)
                .total_cmp(&camera.translation.distance_squared(b.center))
        })
        .map(|(h, _)| h);
    let mut active = 0;
    let mut limbs = 0;
    let mut visible_meshes = 0;
    for (h, herd) in state.herds.iter_mut().enumerate() {
        let distance = camera.translation.distance(herd.center);
        let visible = distance < FAR;
        for (member, a) in herd.animals.iter_mut().enumerate() {
            if a.visible != visible {
                if let Ok(mut v) = visibility.get_mut(a.root) {
                    *v = if visible {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };
                }
                a.visible = visible;
            }
            if !visible {
                continue;
            }
            active += 1;
            let near = nearest == Some(h)
                && if a.near_lod {
                    distance < NEAR + 10.
                } else {
                    distance < NEAR - 10.
                };
            if near != a.near_lod {
                if let Ok(mut v) = visibility.get_mut(a.near) {
                    *v = if near {
                        Visibility::Inherited
                    } else {
                        Visibility::Hidden
                    };
                }
                if let Ok(mut v) = visibility.get_mut(a.far) {
                    *v = if near {
                        Visibility::Hidden
                    } else {
                        Visibility::Inherited
                    };
                }
                a.near_lod = near;
            }
            let phase = rules::phase(h, race.elapsed);
            let f =
                phase.rem_euclid(std::f32::consts::TAU) / std::f32::consts::TAU * SAMPLES as f32;
            let i = f.floor() as usize % SAMPLES;
            let next = (i + 1) % SAMPLES;
            if let Ok(mut t) = transforms.get_mut(a.root) {
                t.translation = a.path[i].lerp(a.path[next], f.fract());
                let d = a.path[next] - a.path[i];
                let yaw = (-12. * phase.sin()).atan2(25. * phase.cos());
                let path_yaw = (a.path[1] - a.path[0]).x.atan2((a.path[1] - a.path[0]).z);
                let local_yaw = (-12. * (std::f32::consts::TAU / SAMPLES as f32 / 2.).sin())
                    .atan2(25. * (std::f32::consts::TAU / SAMPLES as f32 / 2.).cos());
                t.rotation = Quat::from_rotation_y(yaw + path_yaw - local_yaw)
                    * Quat::from_rotation_x(-d.y.atan2(d.x.hypot(d.z)));
                // Shrink into distant grass/haze over 80m, avoiding spawn popping.
                t.scale = Vec3::splat(((FAR - distance) / 80.).clamp(0., 1.));
            }
            visible_meshes += if near { 5 } else { 1 };
            if near {
                for (l, &entity) in a.legs.iter().enumerate() {
                    if let Ok(mut t) = transforms.get_mut(entity) {
                        t.rotation =
                            Quat::from_rotation_x(0.28 * rules::stride(h, member, l, race.elapsed));
                    }
                    limbs += 1;
                }
            }
        }
    }
    state.active_animals = active;
    state.animated_limbs = limbs;
    state.visible_meshes = visible_meshes;
    state.update_micros = start.elapsed().as_secs_f64() * 1e6;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cattle_crossings_keep_a_way_through_each_formation() {
        let track = Track::hills().unwrap();
        assert_eq!(CROSSING_STATIONS, [8870., 11250.]);
        assert_eq!(
            (CROSSING_ROWS, CROSSING_COLUMNS, CROSSING_MEMBERS),
            (4, 5, 20)
        );
        for station in CROSSING_STATIONS {
            let paths =
                std::array::from_fn::<_, CROSSING_MEMBERS, _>(|i| cattle_path(&track, station, i));
            let (point, _) = track.sample_route(0, station, 0.);
            let (_, axis, phase) = paths[0];
            let along = axis.cross(Vec3::Y);
            let span = |row| {
                let members = &paths[row * CROSSING_COLUMNS..(row + 1) * CROSSING_COLUMNS];
                members
                    .iter()
                    .map(|(center, _, _)| (*center - point).dot(axis))
                    .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), x| {
                        (lo.min(x), hi.max(x))
                    })
            };
            for row in 0..CROSSING_ROWS - 1 {
                let (_, hi) = span(row);
                let (lo, _) = span(row + 1);
                // Cow collider is 2.2 m along the crossing axis.
                assert!(lo - hi - 2.2 > 2.8);
                let stagger =
                    paths[(row + 1) * CROSSING_COLUMNS].0 - paths[row * CROSSING_COLUMNS].0;
                assert!(stagger.dot(along).abs() >= 2.75);
            }

            // Sample lanes across the authored bend. One complete lane fits
            // past every row even as the formation crosses it or turns back.
            let lane_spans: Vec<_> = (-24..=24)
                .map(|i| i as f32 * 0.25)
                .filter(|lane| lane.abs() + 0.75 <= track.half_width(station))
                .map(|lane| {
                    (-20..=20)
                        .map(|step| {
                            (track.sample_route(0, station + step as f32, lane).0 - point).dot(axis)
                        })
                        .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), x| {
                            (lo.min(x), hi.max(x))
                        })
                })
                .collect();
            for step in 0..=80 {
                let offset = step as f32 * 0.5 - CROSSING_EXTENT;
                assert!(
                    lane_spans
                        .iter()
                        .any(|&(lo, hi)| paths.iter().all(|(center, _, _)| {
                            let x = (*center - point).dot(axis) + offset;
                            x + 1.85 < lo || x - 1.85 > hi
                        })),
                    "blocked riding line at {station} m, crossing offset {offset} m"
                );
            }

            for (center, path_axis, path_phase) in paths {
                assert_eq!((path_axis, path_phase), (axis, phase));
                for direction in [-1., 1.] {
                    assert!(reserved(
                        &track,
                        center + axis * direction * CROSSING_EXTENT,
                        0.
                    ));
                }
            }
            for heading in [axis, -axis] {
                assert!((cattle_heading(heading) * Vec3::X).dot(heading) > 0.999);
            }
        }
    }

    #[test]
    fn crossing_ground_cache_follows_resident_support() {
        let track = Track::hills().unwrap();
        let _cache = super::super::terrain_cache::Scope::new(&track);
        let mut max_error = 0.0_f32;
        for station in CROSSING_STATIONS {
            for member in 0..CROSSING_MEMBERS {
                let (center, axis, _) = cattle_path(&track, station, member);
                let ground = CrossingGround::from_path(&track, station, center, axis);
                for step in 0..SUPPORT_INTERVALS {
                    let fraction = 2. * (step as f32 + 0.5) / SUPPORT_INTERVALS as f32 - 1.;
                    let p = center + axis * fraction * CROSSING_EXTENT;
                    let exact = super::super::geometry::crossing_support(&track, 0, p, station);
                    max_error = max_error.max((ground.at(fraction) - exact).abs());
                }
            }
        }
        println!("maximum crossing height interpolation error {max_error:.3} m");
        assert!(max_error < 0.2);
    }

    #[test]
    fn swept_herds_clear_roads_and_ground_paths_are_finite() {
        let track = Track::hills().unwrap();
        for (h, &count) in rules::COUNTS.iter().enumerate() {
            for m in 0..count {
                for i in 0..SAMPLES {
                    let p = ground_point(
                        &track,
                        h,
                        m,
                        i as f32 / SAMPLES as f32 * std::f32::consts::TAU,
                    );
                    assert!(p.is_finite());
                    assert!(reserved(&track, p, 0.));
                    assert!(
                        track.scenery_fits(p, 2., 8.),
                        "herd{h} member{m} sample{i} {p:?}"
                    );
                }
            }
        }
    }
}
