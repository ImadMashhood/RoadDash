//! Original sidewalk paving and curbside trees, with a clear pedestrian strip.
use super::layout::WorldLayout;
use crate::{track::Track, world::Obstacle};
use bevy::{camera::visibility::VisibilityRange, prelude::*};
use bevy_rapier3d::prelude::*;
#[derive(Component)]
pub struct StreetTree;
/// Matches the sidewalk curb ramps at junctions.
pub fn sidewalk_point(track: &Track, layout: &WorldLayout, d: f32, lane: f32) -> Vec3 {
    let (p, q) = track.sample(d, 0.0);
    let nearest = layout
        .settings
        .junctions
        .iter()
        .map(|j| (d - j.station_m).abs() - j.half_width_m)
        .fold(f32::INFINITY, f32::min);
    let curb = layout.settings.curb_height_m * (nearest / 5.0).clamp(0.0, 1.0);
    p - q * Vec3::X * lane + Vec3::Y * (curb + 0.025)
}
pub fn tree_sites(track: &Track, layout: &WorldLayout) -> Vec<(f32, Vec3)> {
    let mut sites = Vec::new();
    let mut d = layout.settings.sidewalk_start_m + 12.0;
    while d < layout.settings.sidewalk_end_m - 12.0 {
        if !layout.settings.junction_at(d, 12.0) {
            for side in [-1.0, 1.0] {
                let p = sidewalk_point(track, layout, d, side * (track.half_width(d) + 0.8));
                if !layout.blockers().any(|r| r.contains(p.xz(), 0.65)) {
                    sites.push((d, p));
                }
            }
        }
        d += 44.0;
    }
    sites
}
#[allow(clippy::too_many_arguments)]
fn child(
    commands: &mut Commands,
    owner: Entity,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    position: Vec3,
    scale: Vec3,
) {
    let e = commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position).with_scale(scale),
            VisibilityRange {
                start_margin: 0.0..0.0,
                end_margin: 420.0..470.0,
                use_aabb: false,
            },
        ))
        .id();
    commands.entity(owner).add_child(e);
}
pub fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
    layout: &WorldLayout,
) {
    let joint = materials.add(StandardMaterial {
        base_color: Color::srgb(0.39, 0.39, 0.36),
        perceptual_roughness: 1.0,
        ..default()
    });
    let mut v = Vec::new();
    let mut ix = Vec::new();
    let mut d = layout.settings.sidewalk_start_m;
    while d < layout.settings.sidewalk_end_m {
        if !layout.settings.junction_at(d, 0.08) {
            for side in [-1.0, 1.0] {
                let n = v.len() as u32;
                for (at, width) in [
                    (d - 0.012, 0.25),
                    (d - 0.012, 2.98),
                    (d + 0.012, 2.98),
                    (d + 0.012, 0.25),
                ] {
                    v.push(
                        sidewalk_point(track, layout, at, side * (track.half_width(at) + width))
                            + Vec3::Y * 0.004,
                    );
                }
                let mut faces = [[n, n + 1, n + 2], [n, n + 2, n + 3]];
                if (v[(n + 1) as usize] - v[n as usize])
                    .cross(v[(n + 2) as usize] - v[n as usize])
                    .y
                    < 0.0
                {
                    for face in &mut faces {
                        face.swap(1, 2);
                    }
                }
                ix.extend(faces);
            }
        }
        d += 2.0;
    }
    commands.spawn((
        Name::new("Sidewalk expansion joints"),
        Mesh3d(meshes.add(crate::environment::mesh(&v, &ix))),
        MeshMaterial3d(joint),
        Transform::default(),
    ));
    let trunk = meshes.add(Cylinder::new(0.18, 3.8));
    let crown = meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap());
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let bark = materials.add(Color::srgb(0.24, 0.16, 0.10));
    let soil = materials.add(Color::srgb(0.17, 0.15, 0.12));
    let rim = materials.add(Color::srgb(0.28, 0.29, 0.28));
    let leaves: Vec<_> = [(0.16, 0.31, 0.10), (0.21, 0.37, 0.13), (0.12, 0.27, 0.08)]
        .into_iter()
        .map(|(r, g, b)| {
            materials.add(StandardMaterial {
                base_color: Color::srgb(r, g, b),
                perceptual_roughness: 0.96,
                ..default()
            })
        })
        .collect();
    for (index, (station, p)) in tree_sites(track, layout).into_iter().enumerate() {
        let (_, q) = track.sample(station, 0.0);
        let root = commands
            .spawn((
                Name::new(format!("Street tree {index}")),
                StreetTree,
                Transform::from_translation(p).with_rotation(q),
                Visibility::default(),
                RigidBody::Fixed,
                Collider::compound(vec![
                    (
                        Vec3::Y * 1.9,
                        Quat::IDENTITY,
                        Collider::capsule_y(1.72, 0.18),
                    ),
                    (Vec3::Y * 4.7, Quat::IDENTITY, Collider::ball(1.25)),
                ]),
                Obstacle,
                Friction::coefficient(0.7),
            ))
            .id();
        child(
            commands,
            root,
            trunk.clone(),
            bark.clone(),
            Vec3::Y * 1.9,
            Vec3::ONE,
        );
        child(
            commands,
            root,
            cube.clone(),
            soil.clone(),
            Vec3::Y * 0.006,
            Vec3::new(1.0, 0.012, 1.0),
        );
        for (x, z, sx, sz) in [
            (0.0, -0.54, 1.12, 0.08),
            (0.0, 0.54, 1.12, 0.08),
            (-0.54, 0.0, 0.08, 1.0),
            (0.54, 0.0, 0.08, 1.0),
        ] {
            child(
                commands,
                root,
                cube.clone(),
                rim.clone(),
                Vec3::new(x, 0.012, z),
                Vec3::new(sx, 0.024, sz),
            );
        }
        for (part, offset, scale) in [
            (0, Vec3::new(0.0, 4.7, 0.0), Vec3::new(1.3, 1.5, 1.3)),
            (1, Vec3::new(0.5, 4.5, 0.25), Vec3::splat(1.0)),
            (2, Vec3::new(-0.4, 4.5, -0.3), Vec3::splat(1.05)),
        ] {
            child(
                commands,
                root,
                crown.clone(),
                leaves[(index + part) % leaves.len()].clone(),
                offset,
                scale,
            );
        }
    }
}
