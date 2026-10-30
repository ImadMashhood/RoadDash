use bevy::prelude::*;
use std::f32::consts::TAU;

pub const ROAD_HALF: f32 = 6.5;
pub const LAPS: u32 = 2;
const SAMPLES: usize = 720;

#[derive(Resource)]
pub struct Track {
    pub points: Vec<Vec3>,
    distances: Vec<f32>,
    pub length: f32,
}

impl Default for Track {
    fn default() -> Self {
        let mut points = Vec::new();
        for i in 0..=SAMPLES {
            let a = i as f32 / SAMPLES as f32 * TAU;
            let r = 205.0 + 32.0 * (3.0 * a).sin() + 16.0 * (5.0 * a).cos();
            points.push(Vec3::new(r * a.cos(), 0.10, r * a.sin() * 1.28));
        }
        let mut distances = vec![0.0];
        for pair in points.windows(2) {
            distances.push(distances.last().unwrap() + pair[0].distance(pair[1]));
        }
        let length = *distances.last().unwrap();
        Self {
            points,
            distances,
            length,
        }
    }
}

impl Track {
    /// Arc length sampling keeps speed independent of curve parameterization.
    pub fn sample(&self, distance: f32, lane: f32) -> (Vec3, Quat) {
        let d = distance.rem_euclid(self.length);
        let i = self
            .distances
            .partition_point(|v| *v <= d)
            .saturating_sub(1)
            .min(SAMPLES - 1);
        let t = (d - self.distances[i]) / (self.distances[i + 1] - self.distances[i]);
        let tangent = (self.points[i + 1] - self.points[i]).normalize();
        let right = Vec3::new(tangent.z, 0.0, -tangent.x);
        let pos = self.points[i].lerp(self.points[i + 1], t) + right * lane;
        (pos, Quat::from_rotation_y(tangent.x.atan2(tangent.z)))
    }

    pub fn curvature(&self, d: f32) -> f32 {
        let (_, a) = self.sample(d - 4.0, 0.0);
        let (_, b) = self.sample(d + 4.0, 0.0);
        let va = a * Vec3::Z;
        let vb = b * Vec3::Z;
        va.cross(vb).y.atan2(va.dot(vb)) / 8.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circuit_is_closed_and_wraps_negative_distance() {
        let track = Track::default();
        assert!(track.length > 1200.0);
        assert!(
            track
                .sample(0.0, 0.0)
                .0
                .distance(track.sample(track.length, 0.0).0)
                < 0.001
        );
        assert!(
            track
                .sample(-5.0, 0.0)
                .0
                .distance(track.sample(track.length - 5.0, 0.0).0)
                < 0.001
        );
    }
    #[test]
    fn arc_length_and_lanes_are_consistent() {
        let track = Track::default();
        for d in (0..1400).step_by(13) {
            let (p, q) = track.sample(d as f32, 0.0);
            assert!((p.distance(track.sample(d as f32 + 1.0, 0.0).0) - 1.0).abs() < 0.03);
            assert!((p.distance(track.sample(d as f32, 3.0).0) - 3.0).abs() < 0.001);
            assert!((q.length() - 1.0).abs() < 0.001);
            assert!(track.curvature(d as f32).is_finite());
        }
    }
}
