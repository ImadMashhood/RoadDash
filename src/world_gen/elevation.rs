//! Cheap batched architectural relief for shells outside the authored-detail budget.
//! All vertices are local to the physical lot; these meshes never own collision.
use bevy::prelude::*;
#[derive(Default)]
pub struct Elevation {
    pub glass: Vec<Vec3>,
    pub frames: Vec<Vec3>,
    pub glass_indices: Vec<[u32; 3]>,
    pub frame_indices: Vec<[u32; 3]>,
}
fn quad(vertices: &mut Vec<Vec3>, indices: &mut Vec<[u32; 3]>, points: [Vec3; 4]) {
    let n = vertices.len() as u32;
    vertices.extend(points);
    indices.extend([[n, n + 1, n + 2], [n, n + 2, n + 3]]);
}
impl Elevation {
    pub fn wall(&mut self, a: Vec3, b: Vec3, bottom: f32, top: f32, outward: Vec3) {
        let width = a.distance(b);
        let along = (b - a).normalize_or_zero();
        if width < 1.0 || top - bottom < 2.5 {
            return;
        }
        let columns = (width / 3.0).round().clamp(1.0, 32.0) as usize;
        let floors = ((top - bottom - 0.5) / 3.2).floor().clamp(1.0, 48.0) as usize;
        let pitch = width / columns as f32;
        for floor in 0..floors {
            let y = bottom + 1.8 + floor as f32 * 3.2;
            if y + 0.8 > top {
                continue;
            }
            for col in 0..columns {
                let centre = a + along * (col as f32 + 0.5) * pitch;
                let half = (pitch * 0.28).min(0.82);
                let rectangle = |w: f32, h: f32, depth: f32| {
                    [(-w, -h), (w, -h), (w, h), (-w, h)]
                        .map(|(x, dy)| centre.with_y(y + dy) + along * x + outward * depth)
                };
                let inner = rectangle(half, 0.72, 0.018);
                let outer = rectangle(half + 0.10, 0.82, 0.075);
                // Ensure front winding follows the actual facade side, including bent infill.
                let reverse = along.cross(Vec3::Y).dot(outward) < 0.0;
                let face = |mut p: [Vec3; 4]| {
                    if reverse {
                        p.reverse();
                    }
                    p
                };
                quad(&mut self.glass, &mut self.glass_indices, face(inner));
                for i in 0..4 {
                    let j = (i + 1) % 4;
                    quad(
                        &mut self.frames,
                        &mut self.frame_indices,
                        face([outer[i], outer[j], inner[j], inner[i]]),
                    );
                }
                // Narrow mullion adds window scale even when the full kit is not loaded.
                quad(
                    &mut self.frames,
                    &mut self.frame_indices,
                    face(rectangle(0.035, 0.72, 0.026)),
                );
            }
            self.band(a, b, y + 1.17, 0.10, outward);
        }
        self.band(a, b, top - 0.22, 0.20, outward);
    }
    fn band(&mut self, a: Vec3, b: Vec3, y: f32, height: f32, outward: Vec3) {
        let mut points = [
            a.with_y(y),
            b.with_y(y),
            b.with_y(y + height),
            a.with_y(y + height),
        ]
        .map(|p| p + outward * 0.065);
        if (b - a).cross(Vec3::Y).dot(outward) < 0.0 {
            points.reverse();
        }
        quad(&mut self.frames, &mut self.frame_indices, points);
    }
    pub fn building(size: Vec3) -> Self {
        let mut e = Self::default();
        for i in 0..4 {
            let q = Quat::from_rotation_y(i as f32 * std::f32::consts::FRAC_PI_2);
            let (width, depth) = if i % 2 == 0 {
                (size.x, size.z)
            } else {
                (size.z, size.x)
            };
            e.wall(
                q * Vec3::new(-width / 2.0, 0.0, depth / 2.0),
                q * Vec3::new(width / 2.0, 0.0, depth / 2.0),
                0.0,
                size.y,
                q * Vec3::Z,
            );
        }
        e
    }
    pub fn meshes(self) -> (Mesh, Mesh) {
        (
            crate::environment::mesh(&self.glass, &self.glass_indices),
            crate::environment::mesh(&self.frames, &self.frame_indices),
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tall_shell_relief_has_correct_scale_winding_and_stays_close_to_walls() {
        let size = Vec3::new(20.0, 64.0, 14.0);
        let e = Elevation::building(size);
        assert!(e.glass_indices.len() > 500 && e.frame_indices.len() > e.glass_indices.len() * 4);
        for v in e.glass.iter().chain(&e.frames) {
            assert!(
                v.is_finite()
                    && v.x.abs() <= size.x / 2.0 + 0.08
                    && v.z.abs() <= size.z / 2.0 + 0.08
                    && v.y >= 0.0
                    && v.y <= size.y
            );
        }
        for tri in &e.glass_indices {
            let [a, b, c] = tri.map(|i| e.glass[i as usize]);
            let n = (b - a).cross(c - a);
            let centre = (a + b + c) / 3.0;
            assert!(n.dot(centre.with_y(0.0)) > 0.0);
        }
    }
}
