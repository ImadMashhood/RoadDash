//! Visible timing lines and roadside markers shared by all race courses.
use crate::{track::Track, world::Art};
use bevy::prelude::*;

fn sign_face(label: &str) -> Mesh {
    let mut points = Vec::new();
    let mut triangles = Vec::new();
    let width = (label.len() * 4 - 1) as f32;
    let pitch = 1.26 / width;
    for (index, letter) in label.bytes().enumerate() {
        let rows = match letter {
            b'A' => [2, 5, 7, 5, 5],
            b'F' => [7, 4, 6, 4, 4],
            b'H' => [5, 5, 7, 5, 5],
            b'I' => [7, 2, 2, 2, 7],
            b'L' => [4, 4, 4, 4, 7],
            b'N' => [5, 7, 7, 7, 5],
            b'P' => [6, 5, 6, 4, 4],
            b'R' => [6, 5, 6, 5, 5],
            b'S' => [7, 4, 7, 1, 7],
            b'T' => [7, 2, 2, 2, 2],
            _ => [0; 5],
        };
        for (row, bits) in rows.into_iter().enumerate() {
            for col in 0..3 {
                if bits & (1 << (2 - col)) == 0 {
                    continue;
                }
                let x = (index as f32 * 4. + col as f32 - width * 0.5) * pitch;
                let y = (2.5 - row as f32) * pitch;
                let r = pitch * 0.92;
                let i = points.len() as u32;
                points.extend([
                    Vec3::new(x, y, -0.089),
                    Vec3::new(x + r, y, -0.089),
                    Vec3::new(x + r, y - r, -0.089),
                    Vec3::new(x, y - r, -0.089),
                ]);
                triangles.extend([[i, i + 1, i + 2], [i, i + 2, i + 3]]);
            }
        }
    }
    crate::environment::mesh(&points, &triangles)
}

pub(crate) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    track: &Track,
    art: &Art,
) {
    let interval = track.race_interval();
    let dark = materials.add(Color::srgb(0.026, 0.034, 0.044));
    for (station, label, tint) in if track.definition.closed {
        vec![(
            interval.start,
            "Start / finish",
            Color::srgb(0.95, 0.33, 0.15),
        )]
    } else {
        vec![
            (interval.start, "Start", Color::srgb(0.07, 0.67, 0.58)),
            (interval.finish, "Finish", Color::srgb(0.95, 0.33, 0.15)),
        ]
    } {
        let half = track.half_width(station);
        let center_lane = track.street_offset(0, station);
        let (origin, heading) = track.sample_route(0, station, center_lane);
        let tiles = ((half * 2.).round() as usize).max(2);
        for (parity, material) in [art.white.clone(), dark.clone()].into_iter().enumerate() {
            let mut points = Vec::new();
            let mut triangles = Vec::new();
            let first = if track.definition.closed {
                station - 0.72
            } else {
                (station - 0.72).min(track.length - 1.44)
            };
            for row in 0..2 {
                let behind = first + row as f32 * 0.72;
                for column in 0..tiles {
                    if (row + column) % 2 != parity {
                        continue;
                    }
                    let lane = -half + 2. * half * column as f32 / tiles as f32;
                    let next_lane = lane + 2. * half / tiles as f32;
                    let idx = points.len() as u32;
                    for (s, l) in [
                        (behind, lane),
                        (behind + 0.72, lane),
                        (behind + 0.72, next_lane),
                        (behind, next_lane),
                    ] {
                        let (point, rotation) =
                            track.sample_route(0, s, l + track.street_offset(0, s));
                        points.push(point + rotation * Vec3::Y * 0.065 - origin);
                    }
                    triangles.extend([[idx, idx + 2, idx + 1], [idx, idx + 3, idx + 2]]);
                }
            }
            let a = points[triangles[0][0] as usize];
            let b = points[triangles[0][1] as usize];
            let c = points[triangles[0][2] as usize];
            if (b - a).cross(c - a).y < 0. {
                for tri in &mut triangles {
                    tri.swap(1, 2);
                }
            }
            commands.spawn((
                Name::new(format!("{label} painted timing line")),
                Mesh3d(meshes.add(crate::environment::mesh(&points, &triangles))),
                MeshMaterial3d(material),
                Transform::from_translation(origin),
                bevy::light::NotShadowCaster,
            ));
        }

        let accent = materials.add(tint);
        let text = match label {
            "Start" => "START",
            "Finish" => "FINISH",
            _ => "LAP",
        };
        let face = meshes.add(sign_face(text));
        for side in [-1., 1.] {
            let edge = origin + heading * Vec3::X * side * (half + 1.25);
            let p = edge.with_y(track.terrain_height(edge.x, edge.z).min(origin.y));
            for (height, dimensions, material) in [
                (0.13, Vec3::new(0.95, 0.27, 0.9), dark.clone()),
                (1.43, Vec3::new(0.22, 2.7, 0.26), dark.clone()),
                (1.85, Vec3::new(1.56, 1.02, 0.17), accent.clone()),
                (2.30, Vec3::new(1.57, 0.13, 0.18), art.white.clone()),
            ] {
                commands.spawn((
                    Name::new(format!("{label} roadside marker")),
                    Mesh3d(art.cube.clone()),
                    MeshMaterial3d(material),
                    Transform::from_translation(p + Vec3::Y * height)
                        .with_rotation(heading)
                        .with_scale(dimensions),
                ));
            }
            commands.spawn((
                Name::new(format!("{label} marker lettering")),
                Mesh3d(face.clone()),
                MeshMaterial3d(art.white.clone()),
                Transform::from_translation(p + Vec3::Y * 1.84).with_rotation(heading),
                bevy::light::NotShadowCaster,
            ));
        }
    }
}
