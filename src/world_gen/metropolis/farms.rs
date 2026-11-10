//! Reuse original farm props inside clear field interiors, never on parcel fences.
use super::geometry::landscape::Footprint;
use super::*;

#[derive(Clone, Debug)]
pub struct Placement {
    pub name: &'static str,
    pub position: Vec3,
    pub rotation: Quat,
    pub size: Vec3,
    pub radius: f32,
}
fn clear(track: &Track, p: Vec2, radius: f32, placed: &[Placement]) -> bool {
    let m = track.metropolis.as_ref().unwrap();
    let inside = m.fields.iter().any(|f| {
        let f = Footprint::new(&f.shape);
        f.contains(p) && f.boundary_distance(p) > radius + 1.
    });
    inside
        && track.nearby_station(p.x, p.y).1.abs() > 14. + radius
        && !m
            .surfaces
            .iter()
            .map(|s| &s.shape)
            .chain(m.buildings.iter().map(|b| &b.shape))
            .any(|s| Footprint::new(s).overlaps(p, radius + 1.))
        && ![420., 980., 1560., 2220., 2810.]
            .into_iter()
            .enumerate()
            .any(|(i, s)| {
                track
                    .sample(s, if i % 2 == 0 { -65. } else { 65. })
                    .0
                    .xz()
                    .distance(p)
                    < 14. + radius
            })
        && !placed
            .iter()
            .any(|v| v.position.xz().distance(p) < v.radius + radius + 2.)
}
fn place(
    track: &Track,
    out: &mut Vec<Placement>,
    station: f32,
    side: f32,
    name: &'static str,
    size: Vec3,
    radius: f32,
) {
    for offset in [0., 12., -12., 24., -24., 40., -40.] {
        for lane in [25., 32., 40., 48.] {
            let (p, q) = track.sample(station + offset, side * lane);
            if clear(track, p.xz(), radius, out) {
                out.push(Placement {
                    name,
                    position: p.with_y(track.terrain_height(p.x, p.z) - 0.12),
                    rotation: q,
                    size,
                    radius,
                });
                return;
            }
        }
    }
    panic!("No whole-field clearance for farm {name} at {station}");
}
pub fn plan(track: &Track) -> Vec<Placement> {
    let mut result = Vec::new();
    // Preserve the original cow population and station choices; move whole bodies off fences.
    let path = &track.metropolis.as_ref().unwrap().paths[0];
    for i in (0..path.points.len()).step_by(36) {
        let d = path.progress[i];
        if d < 2700. {
            place(track, &mut result, d, -1., "cow", Vec3::ZERO, 2.5);
        }
    }
    for (i, s) in [180., 900., 1700., 2400.].into_iter().enumerate() {
        let side = if i % 2 == 0 { -1. } else { 1. };
        for (offset, name, size) in [
            (0., "BRNGRN2", Vec3::new(5.08, 3.79, 7.4)),
            (18., "WINDVN2", Vec3::new(2.2, 5.108, 1.23)),
            (-12., "HAYDOUB2", Vec3::new(2.5, 0.66, 1.31)),
            (-18., "HAYTRIP2", Vec3::new(2.5, 0.66, 1.31)),
        ] {
            let radius = size.x.hypot(size.z) * 0.5;
            place(track, &mut result, s + offset, side, name, size, radius);
        }
    }
    result
}
pub fn spawn(commands: &mut Commands, art: &Art, track: &Track, census: &mut Census) {
    let Some(props) = &art.props else { return };
    for p in plan(track) {
        if p.name == "cow" {
            crate::environment::spawn_cow(commands, art, p.position, p.rotation);
            census.farm_cows += 1;
        } else {
            let root = commands
                .spawn((
                    Name::new(format!("Farm {}", p.name)),
                    Transform::from_translation(p.position).with_rotation(p.rotation),
                    Visibility::default(),
                    RigidBody::Fixed,
                    Collider::compound(vec![(
                        Vec3::Y * p.size.y * 0.5,
                        Quat::IDENTITY,
                        Collider::cuboid(p.size.x * 0.5, p.size.y * 0.5, p.size.z * 0.5),
                    )]),
                    Obstacle,
                ))
                .id();
            let visual = props.fitted(
                commands,
                props.named(p.name),
                Vec3::ZERO,
                Quat::IDENTITY,
                p.size,
                "farm",
            );
            crate::runtime::attach_visuals(commands, root, &[visual]);
            census.farm_props += 1;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn whole_cows_and_authored_props_clear_fences_roads_and_each_other() {
        let track = Track::metropolis().unwrap();
        let placements = plan(&track);
        let original = track.metropolis.as_ref().unwrap().paths[0]
            .progress
            .iter()
            .step_by(36)
            .filter(|d| **d < 2700.)
            .count();
        assert_eq!(
            placements.iter().filter(|p| p.name == "cow").count(),
            original
        );
        assert_eq!(placements.len(), original + 16);
        for (i, p) in placements.iter().enumerate() {
            assert!(clear(&track, p.position.xz(), p.radius, &placements[..i]));
        }
        println!("Whole-footprint clear placements: {original} cows and 16 existing farm props");
    }
}
