//! Independently authored desert/forest detail with resident collision.
use super::{HillsSolid, props::Model};
use crate::{track::Track, world::Obstacle};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct HillsDressing {
    pub radius: f32,
    pub kind: &'static str,
}
#[derive(Resource, Default, serde::Serialize)]
pub struct DressingCensus {
    pub cacti: usize,
    pub rocks: usize,
    pub giant_boulders: usize,
    pub deadwood: usize,
}
#[derive(Clone)]
struct Placement {
    p: Vec3,
    q: Quat,
    radius: f32,
    kind: &'static str,
    seed: u32,
}
fn random(seed: u32) -> f32 {
    let h = seed.wrapping_mul(747796405).wrapping_add(2891336453);
    let h = ((h >> ((h >> 28) + 4)) ^ h).wrapping_mul(277803737);
    ((h >> 22) ^ h) as f32 / u32::MAX as f32
}
fn placements(track: &Track) -> Vec<Placement> {
    let mut out: Vec<Placement> = vec![];
    let farms = super::countryside::farm_sites(track);
    for (start, end, step, kind) in [
        (8500, 12450, 31, "cactus"),
        (5550, 8420, 43, "cactus"),
        (5560, 8420, 73, "rock"),
        (1700, 5250, 95, "rock"),
        (1720, 5350, 117, "boulder"),
        (8500, 12450, 113, "rock"),
        (1800, 5200, 170, "deadwood"),
        (8550, 12450, 197, "deadwood"),
    ] {
        for s in (start..end).step_by(step) {
            for side in [-1., 1.] {
                let seed = s as u32 + u32::from(side > 0.) * 777;
                let radius = match kind {
                    "cactus" => 2.8,
                    "boulder" => 9.,
                    "rock" => {
                        if seed.is_multiple_of(3) {
                            4.5
                        } else {
                            1.9
                        }
                    }
                    _ => 4.,
                };
                let lane = side
                    * if s < 5500 {
                        if kind == "boulder" {
                            29. + random(seed) * 48.
                        } else {
                            95. + random(seed) * 35.
                        }
                    } else if s < 8500 {
                        250. + random(seed) * 48.
                    } else {
                        17. + random(seed) * 85.
                    };
                let Some((mut p, q)) = super::props::site(track, 0, s as f32, lane, radius) else {
                    continue;
                };
                if super::bridge::reserved(track, p, radius)
                    || (super::mounted::reserved(track, p, radius)
                        || super::wildlife::reserved(track, p, radius))
                    || farms.iter().any(|(center, q)| {
                        let local = q.inverse() * (p - *center);
                        local.x.abs() < 35. + radius && local.z.abs() < 95. + radius
                    })
                {
                    continue;
                }
                if out
                    .iter()
                    .any(|other| (p - other.p).with_y(0.).length() < radius + other.radius + 2.)
                {
                    continue;
                }
                // Fully sample the footprint. Reject excessive grade; embed the base
                // below its lowest support sample so no cactus foot/rock floats.
                let mut low = p.y;
                let mut high = p.y;
                for i in 0..12 {
                    let a = i as f32 * std::f32::consts::TAU / 12.;
                    let v = p + Vec3::new(a.cos(), 0., a.sin()) * radius;
                    let h = super::geometry::ground_height(track, v.x, v.z);
                    low = low.min(h);
                    high = high.max(h);
                }
                if high - low > if kind == "boulder" { 15. } else { 1.4 } {
                    continue;
                }
                p.y = low - 0.12;
                out.push(Placement {
                    p,
                    q: q * Quat::from_rotation_y(random(seed + 3) * std::f32::consts::TAU),
                    radius,
                    kind,
                    seed,
                });
            }
        }
    }
    out
}
fn cactus(seed: u32) -> (Model, Collider) {
    let mut m = Model::default();
    let mut collision = vec![];
    let h = 2.7 + random(seed) * 2.4;
    let color = [
        0.22 + random(seed + 2) * 0.05,
        0.37 + random(seed + 4) * 0.07,
        0.19,
    ];
    let mut segments = vec![(Vec3::Y * 0.25, Vec3::Y * h, 0.34)];
    for (side, level) in [(-1., 0.48), (1., 0.64)] {
        let a = Vec3::Y * h * level;
        let b = a + Vec3::X * side * (0.9 + random(seed + 5) * 0.4);
        let c = b + Vec3::Y * (0.9 + random(seed + 6) * 0.9);
        segments.extend([(a, b, 0.24), (b, c, 0.24)]);
    }
    for (a, b, r) in segments {
        m.rod(a, b, r, color);
        m.ellipsoid(b, Vec3::splat(r), color, 0.);
        collision.push((Vec3::ZERO, Quat::IDENTITY, Collider::capsule(a, b, r)));
        // Longitudinal ribs stay geometry, with small bright areole marks.
        let q = Quat::from_rotation_arc(Vec3::Y, (b - a).normalize());
        for i in 0..6 {
            let angle = i as f32 * std::f32::consts::TAU / 6.;
            let offset = q * Vec3::new(angle.cos(), 0., angle.sin()) * (r + 0.005);
            m.rod(a + offset, b + offset, 0.022, [0.31, 0.43, 0.23]);
        }
    }
    m.ellipsoid(
        Vec3::Y * (h + 0.34),
        Vec3::new(0.10, 0.14, 0.10),
        [0.64, 0.20, 0.14],
        0.,
    );
    (m, Collider::compound(collision))
}
fn rock(seed: u32) -> (Model, Collider) {
    let mut m = Model::default();
    // Compact, tall solitary stones fit steeper shelves without shrinking the
    // placement envelope of the original three-boulder clusters.
    if !seed.is_multiple_of(3) {
        let radius = 1.0 + random(seed) * 0.4;
        m.ellipsoid(
            Vec3::Y * 0.50,
            Vec3::new(radius, 1.25, radius * 0.85),
            [0.51, 0.43, 0.32],
            1.,
        );
        let collider = m.hull();
        return (m, collider);
    }
    for i in 0..3 {
        let seed = seed + i * 47;
        let radius = 0.65 + random(seed) * 1.15;
        let p = Vec3::new(
            (i as f32 - 1.) * 1.35,
            radius * 0.35,
            random(seed + 3) * 1.3 - 0.65,
        );
        let size = Vec3::new(
            radius,
            radius * (0.65 + random(seed + 7) * 0.4),
            radius * 0.85,
        );
        let color = [
            0.44 + random(seed + 1) * 0.12,
            0.38 + random(seed + 2) * 0.10,
            0.29 + random(seed + 4) * 0.08,
        ];
        m.ellipsoid(p, size, color, 1.);
    }
    let collider = m.hull();
    (m, collider)
}
fn giant_boulder(seed: u32) -> (Model, Collider) {
    let mut m = Model::default();
    let h = 7. + random(seed) * 5.;
    m.ellipsoid(
        Vec3::new(0., h * 0.30, 0.),
        Vec3::new(5.8, h, 5.5),
        [0.42, 0.39, 0.32],
        1.,
    );
    m.ellipsoid(
        Vec3::new(4.2, 0.5, 2.),
        Vec3::new(3.8, 3.8, 3.),
        [0.50, 0.46, 0.36],
        1.,
    );
    m.ellipsoid(
        Vec3::new(-3.5, -0.5, -2.),
        Vec3::new(3.6, 3.2, 3.4),
        [0.37, 0.36, 0.31],
        1.,
    );
    let hull = m.hull();
    (m, hull)
}
fn deadwood(seed: u32) -> (Model, Collider) {
    let mut m = Model::default();
    let color = [0.39, 0.29, 0.18];
    let a = Vec3::new(-2.5, 0.2, 0.);
    let b = Vec3::new(2.5, 0.45, 0.3);
    m.rod(a, b, 0.28, color);
    for (x, z) in [(-1., 1.5), (0.7, -1.3)] {
        m.rod(
            Vec3::new(x, 0.35, 0.),
            Vec3::new(x + 0.5, 0.65, z),
            0.10,
            color,
        );
    }
    m.ellipsoid(
        a,
        Vec3::new(0.12, 0.26, 0.26),
        [0.61, 0.46, 0.28],
        random(seed),
    );
    (
        m,
        Collider::compound(vec![
            (Vec3::ZERO, Quat::IDENTITY, Collider::capsule(a, b, 0.3)),
            (
                Vec3::ZERO,
                Quat::IDENTITY,
                Collider::capsule(Vec3::new(-1., 0.35, 0.), Vec3::new(-0.5, 0.65, 1.5), 0.1),
            ),
            (
                Vec3::ZERO,
                Quat::IDENTITY,
                Collider::capsule(Vec3::new(0.7, 0.35, 0.), Vec3::new(1.2, 0.65, -1.3), 0.1),
            ),
        ]),
    )
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        perceptual_roughness: 0.93,
        ..default()
    });
    let mut census = DressingCensus::default();
    for p in placements(track) {
        let (model, collider) = match p.kind {
            "cactus" => {
                census.cacti += 1;
                cactus(p.seed)
            }
            "boulder" => {
                census.giant_boulders += 1;
                giant_boulder(p.seed)
            }
            "rock" => {
                census.rocks += 1;
                rock(p.seed)
            }
            _ => {
                census.deadwood += 1;
                deadwood(p.seed)
            }
        };
        commands.spawn((
            Name::new(format!("Hills original {}", p.kind)),
            Mesh3d(meshes.add(model.mesh())),
            MeshMaterial3d(material.clone()),
            Transform::from_translation(p.p).with_rotation(p.q),
            RigidBody::Fixed,
            collider,
            Obstacle,
            HillsSolid,
            HillsDressing {
                radius: p.radius,
                kind: p.kind,
            },
        ));
    }
    commands.insert_resource(census);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn desert_and_forest_details_have_supported_nonoverlapping_road_clear_footprints() {
        let track = Track::hills().unwrap();
        let sites = placements(&track);
        let counts = ["cactus", "rock", "deadwood"]
            .map(|kind| sites.iter().filter(|p| p.kind == kind).count());
        let forest_rocks = sites
            .iter()
            .filter(|p| p.kind == "rock" && track.project_route(0, p.p, 0.).0 < 5500.)
            .count();
        eprintln!(
            "Hills dressing counts cactus/rock/deadwood: {counts:?}; forest rocks: {forest_rocks}"
        );
        assert!(
            forest_rocks > 15,
            "steep hills need placed rocks, not only dune coverage"
        );
        assert!(
            sites.iter().filter(|p| p.kind == "boulder").count() >= 8,
            "giant hill boulders need visible coverage"
        );
        assert!(sites.iter().filter(|p| p.kind == "cactus").count() > 70);
        assert!(sites.iter().filter(|p| p.kind == "rock").count() > 35);
        assert!(sites.iter().filter(|p| p.kind == "deadwood").count() > 10);
        for (i, p) in sites.iter().enumerate() {
            for route in 0..2 {
                let (s, _, distance) = track.project_route(route, p.p, 0.);
                assert!(distance > track.half_width(s) + p.radius + 2.9);
            }
            for other in &sites[..i] {
                assert!((p.p - other.p).with_y(0.).length() > p.radius + other.radius + 2.);
            }
        }
    }
}
