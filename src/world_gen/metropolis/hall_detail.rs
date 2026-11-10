//! Original civic stonework, assembled into five resident material batches.
//! This is a presentation layer over civic.rs's unchanged solid architecture.
use super::*;
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
};

#[derive(Default)]
struct Batch {
    p: Vec<[f32; 3]>,
    n: Vec<[f32; 3]>,
    indices: Vec<u32>,
    uv: Vec<[f32; 2]>,
}
impl Batch {
    fn block(&mut self, p: Vec3, size: Vec3, q: Quat) {
        for (normal, u, v) in [
            (Vec3::X, Vec3::Y, Vec3::Z),
            (-Vec3::X, Vec3::Z, Vec3::Y),
            (Vec3::Y, Vec3::Z, Vec3::X),
            (-Vec3::Y, Vec3::X, Vec3::Z),
            (Vec3::Z, Vec3::X, Vec3::Y),
            (-Vec3::Z, Vec3::Y, Vec3::X),
        ] {
            let start = self.p.len() as u32;
            for (a, b) in [(-1., -1.), (1., -1.), (1., 1.), (-1., 1.)] {
                self.p
                    .push((p + q * ((normal + u * a + v * b) * size * 0.5)).to_array());
                self.n.push((q * normal).to_array());
                // Local face axes rotate with the block; actual world metres
                // keep narrow trim and broad roof sheets at the same density.
                // Position offsets decorrelate adjoining cut stone pieces.
                let offset = Vec2::new(p.dot(q * u), p.dot(q * v));
                self.uv.push(
                    ((offset + Vec2::new(a * size.dot(u), b * size.dot(v)) * 0.5)
                        * super::civic::HALL_SCALE
                        / super::hall_surface::TILE_METERS)
                        .to_array(),
                );
            }
            self.indices
                .extend([start, start + 1, start + 2, start, start + 2, start + 3]);
        }
    }
    fn bar(&mut self, a: Vec3, b: Vec3, width: f32, depth: f32) {
        let d = b - a;
        self.block(
            (a + b) * 0.5,
            Vec3::new(width, d.length(), depth),
            Quat::from_rotation_arc(Vec3::Y, d.normalize()),
        );
    }
    fn mesh(self) -> Mesh {
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.p)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.n)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uv)
        .with_inserted_indices(Indices::U32(self.indices));
        mesh.generate_tangents()
            .expect("nondegenerate civic face UVs");
        mesh
    }
}
fn build() -> [Batch; 5] {
    let [mut stone, mut pale, mut dark, mut copper, mut brass] =
        std::array::from_fn(|_| Batch::default());
    let q = Quat::IDENTITY;
    // Rusticated podium and belt courses wrap the closed wall envelope.
    for (cx, w, h) in [(-23., 17., 14.), (0., 28., 20.), (23., 17., 14.)] {
        for y in [1.4, 2.1, 7., h - 0.7, h + 0.35] {
            pale.block(Vec3::new(cx, y, -4.), Vec3::new(w + 0.35, 0.25, 29.5), q);
        }
        for z in [-18.65, 10.65] {
            for x in ((cx - w * 0.5 + 0.7) as i32..=(cx + w * 0.5) as i32).step_by(3) {
                for y in [2.7, 3.6, 4.5, 5.4, 6.3] {
                    stone.block(Vec3::new(x as f32, y, z), Vec3::new(2.85, 0.83, 0.22), q);
                }
            }
        }
        // Long low copper roof slopes: visible shoulders, not flat slabs.
        for side in [-1., 1.] {
            copper.block(
                Vec3::new(cx, h + 1.45, -4. + side * 7.),
                Vec3::new(w, 0.24, 14.8),
                Quat::from_rotation_x(side * 0.12),
            );
        }
        // Raised standing seams break broad roofs into plausible copper sheets.
        for strip in 0..((w / 2.) as i32) {
            let x = cx - w * 0.5 + 1. + strip as f32 * 2.;
            for side in [-1., 1.] {
                copper.block(
                    Vec3::new(x, h + 1.6, -4. + side * 7.),
                    Vec3::new(0.055, 0.12, 14.8),
                    Quat::from_rotation_x(side * 0.12),
                );
            }
        }
        copper.block(
            Vec3::new(cx, h + 2.32, -4.),
            Vec3::new(w + 0.3, 0.25, 0.4),
            q,
        );
    }
    // Recess illusion uses deep shadow reveals, projecting jambs and mullions.
    for (origin, rot, count) in [
        (Vec3::new(0., 0., 10.88), q, 10),
        (
            Vec3::new(0., 0., -18.88),
            Quat::from_rotation_y(std::f32::consts::PI),
            10,
        ),
        (
            Vec3::new(31.8, 0., -4.),
            Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
            5,
        ),
        (
            Vec3::new(-31.8, 0., -4.),
            Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
            5,
        ),
    ] {
        for i in 0..count {
            let x = (i as f32 - (count - 1) as f32 * 0.5) * 6.;
            for y in [4., 10.] {
                let at = |dx: f32, dy: f32, z: f32| origin + rot * Vec3::new(x + dx, y + dy, z);
                dark.block(at(0., 0., 0.), Vec3::new(2.7, 4.1, 0.12), rot);
                // Muted blue-green panes sit behind the projecting stone surround.
                copper.block(at(0., 0., 0.08), Vec3::new(2.15, 3.5, 0.1), rot);
                for side in [-1., 1.] {
                    pale.block(at(side * 1.35, 0., 0.2), Vec3::new(0.27, 4.2, 0.5), rot);
                }
                for yy in [-2.05, 2.05] {
                    pale.block(at(0., yy, 0.25), Vec3::new(3.1, 0.28, 0.65), rot);
                }
                pale.block(at(0., 2.35, 0.32), Vec3::new(3.5, 0.22, 0.8), rot);
                brass.block(at(0., 0., 0.18), Vec3::new(0.09, 3.6, 0.12), rot);
                brass.block(at(0., 0.3, 0.18), Vec3::new(2.2, 0.09, 0.12), rot);
            }
        }
    }
    // The taller central chamber receives the same complete window treatment
    // on both elevations, including the rear visible over the copper wings.
    for (z, rot) in [
        (10.88, q),
        (-18.88, Quat::from_rotation_y(std::f32::consts::PI)),
    ] {
        for x in [-10., -5., 0., 5., 10.] {
            let at = |dx: f32, dy: f32, dz: f32| Vec3::new(x, 16., z) + rot * Vec3::new(dx, dy, dz);
            dark.block(at(0., 0., 0.), Vec3::new(2.8, 4.5, 0.12), rot);
            copper.block(at(0., 0., 0.09), Vec3::new(2.3, 3.95, 0.1), rot);
            for side in [-1., 1.] {
                pale.block(at(side * 1.42, 0., 0.22), Vec3::new(0.28, 4.6, 0.5), rot);
            }
            for yy in [-2.2, 2.2] {
                pale.block(at(0., yy, 0.25), Vec3::new(3.3, 0.3, 0.65), rot);
            }
            pale.block(at(0., 2.5, 0.3), Vec3::new(3.6, 0.24, 0.8), rot);
            brass.block(at(0., 0., 0.19), Vec3::new(0.1, 4., 0.12), rot);
            brass.block(at(0., 0.3, 0.19), Vec3::new(2.3, 0.1, 0.12), rot);
        }
    }
    // Bronze double doors sit behind the open middle arch, at the retained wall.
    dark.block(Vec3::new(0., 3.5, 10.97), Vec3::new(4.4, 5.4, 0.16), q);
    for side in [-1., 1.] {
        for y in [2., 4.4] {
            brass.block(
                Vec3::new(side * 1.03, y, 11.1),
                Vec3::new(1.82, 2.1, 0.12),
                q,
            );
            dark.block(
                Vec3::new(side * 1.03, y, 11.19),
                Vec3::new(1.52, 1.8, 0.08),
                q,
            );
        }
        pale.block(
            Vec3::new(side * 2.4, 3.5, 11.1),
            Vec3::new(0.45, 5.7, 0.65),
            q,
        );
        brass.block(
            Vec3::new(side * 0.2, 3.2, 11.3),
            Vec3::new(0.09, 0.65, 0.18),
            q,
        );
    }
    pale.block(Vec3::new(0., 6.4, 11.2), Vec3::new(5.4, 0.4, 0.8), q);
    // Corner quoins and rhythmical upper pilasters articulate the large masses.
    for x in [-31.3, -14.1, 14.1, 31.3] {
        for z in [-18.65, 10.7] {
            for i in 0..12 {
                pale.block(
                    Vec3::new(x, 1.7 + i as f32 * 1.05, z),
                    Vec3::new(if i % 2 == 0 { 1.4 } else { 1. }, 0.91, 0.5),
                    q,
                );
            }
        }
    }
    for x in [-12., -7.5, -2.5, 2.5, 7.5, 12.] {
        pale.block(Vec3::new(x, 16., 10.85), Vec3::new(0.45, 5.6, 0.5), q);
        pale.block(Vec3::new(x, 19., 11.), Vec3::new(0.9, 0.45, 0.7), q);
    }
    // The open arcade stays open. Add capitals, keystones and a balustraded terrace.
    for bay in -3..=3 {
        let x = bay as f32 * 8.;
        for side in [-1., 1.] {
            pale.block(
                Vec3::new(x + side * 3.4, 5.8, 17.),
                Vec3::new(1.65, 0.4, 2.25),
                q,
            );
            stone.block(
                Vec3::new(x + side * 3.4, 3.3, 18.),
                Vec3::new(0.7, 4.4, 0.16),
                q,
            );
        }
        pale.block(Vec3::new(x, 9.25, 18.05), Vec3::new(0.75, 1.05, 0.4), q);
    }
    for (y, height, depth) in [(10.8, 0.25, 7.5), (11.15, 0.22, 7.8), (12.6, 0.25, 0.65)] {
        pale.block(
            Vec3::new(0., y, if y > 12. { 18.25 } else { 15. }),
            Vec3::new(62.4, height, depth),
            q,
        );
    }
    for i in -38..=38 {
        pale.block(
            Vec3::new(i as f32 * 0.8, 11.9, 18.25),
            Vec3::new(0.23, 1.25, 0.3),
            q,
        );
    }
    for x in [-30., -16., 0., 16., 30.] {
        pale.block(Vec3::new(x, 11.85, 18.25), Vec3::new(0.8, 1.6, 0.9), q);
    }
    // Pediment raking cornices and a geometric civic sunburst relief.
    for z in [13.15, 13.45] {
        for side in [-1., 1.] {
            pale.bar(
                Vec3::new(side * 15.1, 21., z),
                Vec3::new(0., 27.15, z),
                0.4,
                0.45,
            );
        }
        pale.block(Vec3::new(0., 21.2, z), Vec3::new(30.5, 0.4, 0.5), q);
    }
    for i in 0..12 {
        let a = i as f32 * std::f32::consts::TAU / 12.;
        brass.bar(
            Vec3::new(a.cos() * 0.7, 23.1 + a.sin() * 0.7, 13.6),
            Vec3::new(a.cos() * 1.25, 23.1 + a.sin() * 1.25, 13.6),
            0.13,
            0.15,
        );
    }
    // Four public clock faces, tick marks and layered tower cornices.
    // Authored face depth in metres; this dimension is unrelated to an angle.
    #[allow(clippy::approx_constant)]
    const CLOCK_TICK_DEPTH_M: f32 = 6.28;
    for y in [22., 25., 33.7, 34.3, 35.3] {
        pale.block(Vec3::new(0., y, -4.), Vec3::new(12.7, 0.3, 12.7), q);
    }
    for x in [-5.8, 5.8] {
        for z in [-9.8, 1.8] {
            pale.block(Vec3::new(x, 28., z), Vec3::new(0.55, 11., 0.55), q);
        }
    }
    for face in 0..4 {
        let rot = Quat::from_rotation_y(face as f32 * std::f32::consts::FRAC_PI_2);
        let at = |x: f32, y: f32, z: f32| Vec3::new(0., 29., -4.) + rot * Vec3::new(x, y, z);
        dark.block(at(0., 0., 6.1), Vec3::new(6.5, 6.5, 0.2), rot);
        for i in 0..48 {
            let a = i as f32 * std::f32::consts::TAU / 48.;
            pale.block(
                at(3.05 * a.sin(), 3.05 * a.cos(), 6.25),
                Vec3::new(0.43, 0.42, 0.3),
                rot * Quat::from_rotation_z(-a),
            );
            if i % 4 == 0 {
                brass.block(
                    at(2.45 * a.sin(), 2.45 * a.cos(), CLOCK_TICK_DEPTH_M),
                    Vec3::new(0.16, 0.45, 0.15),
                    rot * Quat::from_rotation_z(-a),
                );
            }
        }
        brass.bar(at(0., 0., 6.4), at(-1.35, 0.85, 6.4), 0.16, 0.16);
        brass.bar(at(0., 0., 6.4), at(0., 2.1, 6.4), 0.12, 0.16);
    }
    // Copper lantern: rhythmic dark openings and slender bronze ribs turn the
    // retained circular solid crown into a legible civic roof pavilion.
    for i in 0..32 {
        let a = i as f32 * std::f32::consts::TAU / 32.;
        let rot = Quat::from_rotation_y(a);
        let at = |r: f32, y: f32| Vec3::new(r * a.sin(), y, -4. + r * a.cos());
        dark.block(at(7.015, 39.), Vec3::new(1.14, 3.5, 0.12), rot);
        brass.block(at(7.12, 39.), Vec3::new(0.16, 4.35, 0.16), rot);
        for y in [36.65, 37., 41., 41.45] {
            copper.block(at(7.08, y), Vec3::new(1.48, 0.18, 0.35), rot);
        }
    }
    [stone, pale, dark, copper, brass]
}
pub(super) fn spawn(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    root: Entity,
    images: &mut Assets<Image>,
) {
    let surfaces = super::hall_surface::materials(images);
    for (i, (batch, material)) in build().into_iter().zip(surfaces).enumerate() {
        let child = commands
            .spawn((
                Name::new(format!("Hall stonework batch {i}")),
                Mesh3d(meshes.add(batch.mesh())),
                MeshMaterial3d(materials.add(material)),
                Transform::default(),
            ))
            .id();
        commands.entity(root).add_child(child);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resident_stonework_is_bounded_and_outward_wound() {
        let batches = build();
        let triangles: usize = batches.iter().map(|b| b.indices.len() / 3).sum();
        assert!(triangles < 25_000, "hall ornament budget: {triangles}");
        for batch in batches {
            assert!(!batch.p.is_empty());
            assert_eq!(batch.p.len(), batch.uv.len());
            for (face, uv) in batch.p.chunks_exact(4).zip(batch.uv.chunks_exact(4)) {
                for edge in [1, 3] {
                    let metres = Vec3::from_array(face[edge]).distance(Vec3::from_array(face[0]))
                        * super::super::civic::HALL_SCALE;
                    let tiles = Vec2::from_array(uv[edge]).distance(Vec2::from_array(uv[0]));
                    assert!(
                        (metres - tiles * super::super::hall_surface::TILE_METERS).abs() < 0.0001
                    );
                }
            }
            for p in &batch.p {
                let p = Vec3::from_array(*p);
                assert!(p.is_finite());
                assert!(p.x.abs() < 33. && p.z > -20. && p.z < 20.);
                assert!(p.y > 0. && p.y < 42.);
            }
            for t in batch.indices.chunks_exact(3) {
                let [a, b, c] = [t[0], t[1], t[2]].map(|i| Vec3::from_array(batch.p[i as usize]));
                let normal = Vec3::from_array(batch.n[t[0] as usize]);
                assert!((b - a).cross(c - a).dot(normal) > 0.00001);
            }
        }
        println!("Hall detail: five resident meshes, {triangles} triangles; no collider changes");
    }
}
