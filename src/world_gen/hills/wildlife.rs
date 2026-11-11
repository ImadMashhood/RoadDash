//! Bounded, non-contact off-road herds. No authoritative physics transforms.
use super::{Track, props::Model};
use bevy::prelude::*;
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
pub const NEAR: f32 = 100.;
pub const FAR: f32 = 500.;
#[derive(Component)]
pub struct WildAnimal {
    pub herd: usize,
    pub member: usize,
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
    })
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
