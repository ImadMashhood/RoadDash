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
        let right = Vec3::new(-tangent.z, 0.0, tangent.x);
        let pos = self.points[i].lerp(self.points[i + 1], t) + right * lane;
        (pos, Quat::from_rotation_y(tangent.x.atan2(tangent.z)))
    }

    /// Nearest point on the full polyline, with a signed camera-right lane.
    pub fn project(&self, point: Vec3, near: f32) -> (f32, f32, f32) {
        let mut best = (0.0, 0.0, f32::INFINITY);
        for (i, pair) in self.points.windows(2).enumerate() {
            let a = pair[0].with_y(0.0);
            let edge = (pair[1] - pair[0]).with_y(0.0);
            let t = ((point.with_y(0.0) - a).dot(edge) / edge.length_squared()).clamp(0.0, 1.0);
            let delta = point.with_y(0.0) - (a + edge * t);
            let distance = delta.length();
            if distance < best.2 {
                let d = self.distances[i] + t * edge.length();
                let progress = near + (d - near + self.length * 0.5).rem_euclid(self.length)
                    - self.length * 0.5;
                let right = Vec3::new(-edge.z, 0.0, edge.x).normalize();
                best = (progress, delta.dot(right), distance);
            }
        }
        best
    }

    pub fn scenery_fits(&self, center: Vec3, radius: f32, margin: f32) -> bool {
        self.project(center, 0.0).2 > ROAD_HALF + radius + margin
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
    #[test]
    fn positive_lane_is_camera_right_and_projection_preserves_laps() {
        let track = Track::default();
        for d in (0..1600).step_by(17) {
            let d = d as f32;
            let (p, rotation) = track.sample(d, 0.0);
            let screen_right = (rotation * Vec3::Z).cross(Vec3::Y);
            assert!((track.sample(d, 2.0).0 - p).dot(screen_right) > 1.99);
            let target = d + track.length * 2.0;
            let (distance, lane, _) = track.project(track.sample(target, 2.0).0, target);
            assert!(
                (distance - target).abs() < 0.2,
                "projection changed lap/progress: {distance} vs {target}"
            );
            assert!((lane - 2.0).abs() < 0.05);
            assert!(
                track
                    .sample(distance, lane)
                    .0
                    .distance(track.sample(target, 2.0).0)
                    < 0.2
            );
        }
    }
}
