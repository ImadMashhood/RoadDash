use bevy::prelude::*;
use rr_content::track::{Sector, Surface, TrackDef};
pub const ROAD_HALF: f32 = 6.5;
pub const LAPS: u32 = 1;
#[derive(Resource)]
pub struct Track {
    pub definition: TrackDef,
    pub points: Vec<Vec3>,
    pub distances: Vec<f32>,
    widths: Vec<f32>,
    banks: Vec<f32>,
    sectors: Vec<Sector>,
    surfaces: Vec<Surface>,
    pub length: f32,
    pub bounds: (Vec3, Vec3),
}
impl Default for Track {
    fn default() -> Self {
        Self::from_json(include_str!("../content/tracks/handling.json"))
            .expect("built-in handling road")
    }
}
impl Track {
    pub fn selected() -> Self {
        if let Some(path) = std::env::var_os("ROADDASH_TRACK") {
            Self::from_json(&std::fs::read_to_string(path).expect("read ROADDASH_TRACK"))
                .expect("validate ROADDASH_TRACK")
        } else {
            Self::from_json(include_str!("../content/tracks/valley.json"))
                .expect("built-in scenic road")
        }
    }
    pub fn from_json(json: &str) -> Result<Self, String> {
        Self::from_definition(TrackDef::from_json(json)?)
    }
    pub fn from_definition(definition: TrackDef) -> Result<Self, String> {
        definition.validate()?;
        let cp = &definition.control_points;
        let n = cp.len();
        let mut points = Vec::new();
        let mut widths = Vec::new();
        let mut banks = Vec::new();
        let mut sectors = Vec::new();
        let mut surfaces = Vec::new();
        for i in 0..if definition.closed { n } else { n - 1 } {
            let p0 = if !definition.closed && i == 0 {
                2.0 * Vec3::from_array(cp[0].position) - Vec3::from_array(cp[1].position)
            } else {
                Vec3::from_array(cp[(i + n - 1) % n].position)
            };
            let p1 = Vec3::from_array(cp[i].position);
            let p2 = Vec3::from_array(cp[(i + 1) % n].position);
            let p3 = if !definition.closed && i + 2 == n {
                2.0 * p2 - p1
            } else {
                Vec3::from_array(cp[(i + 2) % n].position)
            };
            let count = (p1.distance(p2) / 1.25).ceil() as usize;
            for j in 0..count {
                let t = j as f32 / count as f32;
                let p = 0.5
                    * ((2.0 * p1)
                        + (-p0 + p2) * t
                        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
                        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t);
                if !p.is_finite() {
                    return Err("nonfinite spline".into());
                }
                points.push(p);
                widths.push(cp[i].half_width.lerp(cp[(i + 1) % n].half_width, t));
                banks.push(
                    cp[i]
                        .bank_degrees
                        .lerp(cp[(i + 1) % n].bank_degrees, t)
                        .to_radians(),
                );
                sectors.push(cp[i].sector);
                surfaces.push(cp[i].surface);
            }
        }
        let end = if definition.closed {
            &cp[0]
        } else {
            &cp[n - 1]
        };
        points.push(Vec3::from_array(end.position));
        widths.push(end.half_width);
        banks.push(end.bank_degrees.to_radians());
        sectors.push(end.sector);
        surfaces.push(end.surface);
        let mut distances = vec![0.0];
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);
        for pair in points.windows(2) {
            let span = pair[0].distance(pair[1]);
            if span < 0.001 {
                return Err("degenerate spline".into());
            }
            distances.push(distances.last().unwrap() + span);
            min = min.min(pair[0]);
            max = max.max(pair[0]);
        }
        let length = *distances.last().unwrap();
        if !(100.0..=20000.0).contains(&length) {
            return Err("route length outside 100 m to 20 km".into());
        }

        let track = Self {
            definition,
            points,
            distances,
            widths,
            banks,
            sectors,
            surfaces,
            length,
            bounds: (min, max),
        };
        for spawn in &track.definition.spawns {
            if spawn.lane.abs() + 0.6 > track.half_width(spawn.distance) {
                return Err("spawn outside safe road width".into());
            }
        }
        for prop in &track.definition.props {
            if matches!(
                prop.kind,
                rr_content::track::PropKind::Building | rr_content::track::PropKind::Farm
            ) && !track.scenery_fits(
                track.sample(prop.fraction * track.length, prop.lane).0,
                12.0,
                2.0,
            ) {
                return Err("authored building overlaps the road".into());
            }
        }
        // Coarse continuous-segment test rejects spline self-crossings; nearby
        // adjacent samples are excluded, grade-separated segments are permitted.
        let coarse: Vec<_> = (0..(length / 10.0) as usize)
            .map(|i| track.sample(i as f32 * 10.0, 0.0).0)
            .collect();
        for i in 0..coarse.len() - 1 {
            for j in i + 3..coarse.len() - 1 {
                let (a, b, c, d) = (coarse[i], coarse[i + 1], coarse[j], coarse[j + 1]);
                let cross = |u: Vec3, v: Vec3| u.x * v.z - u.z * v.x;
                if cross(b - a, c - a) * cross(b - a, d - a) < 0.0
                    && cross(d - c, a - c) * cross(d - c, b - c) < 0.0
                    && ((a.y + b.y) - (c.y + d.y)).abs() < 10.0
                {
                    return Err("self-crossing centerline/checkpoint corridor".into());
                }
            }
        }
        Ok(track)
    }
    fn segment(&self, d: f32) -> (usize, f32) {
        let d = if self.definition.closed {
            d.rem_euclid(self.length)
        } else {
            d.clamp(0.0, self.length)
        };
        let i = self
            .distances
            .partition_point(|x| *x <= d)
            .saturating_sub(1)
            .min(self.points.len() - 2);
        (
            i,
            (d - self.distances[i]) / (self.distances[i + 1] - self.distances[i]),
        )
    }
    pub fn half_width(&self, d: f32) -> f32 {
        let (i, t) = self.segment(d);
        self.widths[i].lerp(self.widths[i + 1], t)
    }
    pub fn sector(&self, d: f32) -> Sector {
        self.sectors[self.segment(d).0]
    }
    pub fn surface(&self, d: f32, lane: f32) -> Surface {
        if lane.abs() > self.half_width(d) + 1.5 {
            Surface::Grass
        } else if lane.abs() > self.half_width(d) {
            Surface::Gravel
        } else {
            self.surfaces[self.segment(d).0]
        }
    }
    pub fn sample(&self, d: f32, lane: f32) -> (Vec3, Quat) {
        let (i, t) = self.segment(d);
        let forward = (self.points[i + 1] - self.points[i]).normalize();
        let side = Vec3::Y.cross(forward).normalize();
        let up = forward.cross(side);
        let q = Quat::from_mat3(&Mat3::from_cols(side, up, forward))
            * Quat::from_rotation_z(self.banks[i].lerp(self.banks[i + 1], t));
        let mut p = self.points[i].lerp(self.points[i + 1], t) - q * Vec3::X * lane;
        if !self.definition.closed {
            p += forward
                * if d < 0.0 {
                    d
                } else if d > self.length {
                    d - self.length
                } else {
                    0.0
                };
        }
        if lane.abs() > self.half_width(d) + self.definition.verge_width {
            p.y = self.terrain_height(p.x, p.z) + 0.1;
        }
        (p, q)
    }
    pub fn project(&self, point: Vec3, near: f32) -> (f32, f32, f32) {
        let mut best = (0.0, 0.0, f32::INFINITY);
        for (i, pair) in self.points.windows(2).enumerate() {
            let edge = (pair[1] - pair[0]).with_y(0.0);
            let t =
                ((point - pair[0]).with_y(0.0).dot(edge) / edge.length_squared()).clamp(0.0, 1.0);
            let delta = (point - pair[0].lerp(pair[1], t)).with_y(0.0);
            let distance = delta.length();
            if distance < best.2 {
                let d = self.distances[i] + t * (self.distances[i + 1] - self.distances[i]);
                let progress = if self.definition.closed {
                    near + (d - near + self.length * 0.5).rem_euclid(self.length)
                        - self.length * 0.5
                } else {
                    d
                };
                let right = Vec3::new(-edge.z, 0.0, edge.x).normalize();
                best = (progress, delta.dot(right), distance);
            }
        }
        best
    }
    /// Single-valued terrain shared by mesh collision and off-road controller.
    /// A flat safe verge follows each road; distant hills blend smoothly away.
    pub fn terrain_height(&self, x: f32, z: f32) -> f32 {
        let point = Vec3::new(x, 0.0, z);
        let mut best = (0, f32::INFINITY);
        for i in (0..self.points.len() - 1).step_by(12) {
            let dist = (self.points[i] - point).with_y(0.0).length_squared();
            if dist < best.1 {
                best = (i, dist);
            }
        }
        let mut closest = Vec3::ZERO;
        let mut lateral_height = 0.0;
        let mut distance = f32::INFINITY;
        let start = best.0.saturating_sub(12);
        let end = (best.0 + 13).min(self.points.len() - 1);
        for i in start..end {
            let edge = (self.points[i + 1] - self.points[i]).with_y(0.0);
            let t = ((point - self.points[i]).dot(edge) / edge.length_squared()).clamp(0.0, 1.0);
            let p = self.points[i].lerp(self.points[i + 1], t);
            let d = (point - p).with_y(0.0).length();
            if d < distance {
                distance = d;
                closest = p;
                let side = Vec3::Y.cross(edge).normalize();
                lateral_height =
                    (point - p).dot(side) * self.banks[i].lerp(self.banks[i + 1], t).tan();
            }
        }
        let blend = ((distance - 18.0) / 100.0).clamp(0.0, 1.0);
        let blend = blend * blend * (3.0 - 2.0 * blend);
        let hill = ((z - 440.0) / 260.0).clamp(0.0, 1.0);
        let remote = hill * (42.0 + 25.0 * (z * 0.009).sin() + 18.0 * (x * 0.012).cos());
        (closest.y + lateral_height - 0.35).lerp(remote, blend)
    }
    pub fn scenery_fits(&self, center: Vec3, radius: f32, margin: f32) -> bool {
        let (d, _, distance) = self.project(center, 0.0);
        distance > self.half_width(d) + radius + margin
    }
    pub fn curvature(&self, d: f32) -> f32 {
        let (_, a) = self.sample(d - 4.0, 0.0);
        let (_, b) = self.sample(d + 4.0, 0.0);
        let va = (a * Vec3::Z).with_y(0.0).normalize();
        let vb = (b * Vec3::Z).with_y(0.0).normalize();
        va.cross(vb).y.atan2(va.dot(vb)) / 8.0
    }
}
/// Ordered physical gates. Progress alone (teleport, reverse crossing, shortcut)
/// cannot complete a lap. Crossing tolerance includes the walkable verge.
#[derive(Clone, Debug)]
pub struct GateProgress {
    pub next: usize,
    pub completed_laps: u32,
}
impl Default for GateProgress {
    fn default() -> Self {
        Self {
            next: 1,
            completed_laps: 0,
        }
    }
}
impl GateProgress {
    pub fn observe(&mut self, track: &Track, from: Vec3, to: Vec3, before: f32, after: f32) {
        if after <= before || after - before > 30.0 {
            return;
        }
        let gate = &track.definition.checkpoints[self.next];
        let base = gate.fraction * track.length;
        let target = if track.definition.closed {
            base + ((before - base) / track.length).ceil() * track.length
        } else {
            base
        };
        if before > target || after < target {
            return;
        }
        let (p, q) = track.sample(target, 0.0);
        let forward = q * Vec3::Z;
        let a = (from - p).dot(forward);
        let b = (to - p).dot(forward);
        if a > 0.1 || b < -0.1 || b - a < 0.0001 {
            return;
        }
        let point = from.lerp(to, (-a / (b - a)).clamp(0.0, 1.0));
        if (point - p).dot(q * Vec3::X).abs()
            > track.half_width(target) + track.definition.verge_width
            || (point - p).dot(q * Vec3::Y).abs() > 4.0
        {
            return;
        }
        if !track.definition.closed && self.next + 1 == track.definition.checkpoints.len() {
            self.completed_laps = 1;
        } else if self.next == 0 {
            self.completed_laps += 1;
            self.next = 1;
        } else {
            self.next = (self.next + 1) % track.definition.checkpoints.len();
        }
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
    #[test]
    fn authored_roads_load_with_elevation_banking_and_canonical_distance() {
        for (json, min, max) in [
            (
                include_str!("../content/tracks/handling.json"),
                1900.0,
                2100.0,
            ),
            (
                include_str!("../content/tracks/valley.json"),
                6000.0,
                9000.0,
            ),
        ] {
            let t = Track::from_json(json).unwrap();
            assert!((min..max).contains(&t.length), "length {}", t.length);
            for i in 0..100 {
                let d = i as f32 * t.length / 100.0;
                let (p, q) = t.sample(d, 0.0);
                assert!(p.is_finite() && q.is_finite());
                assert!(q.dot(t.sample(d + 0.1, 0.0).1).abs() > 0.99);
                let (projected, _, _) = t.project(p, d);
                assert!((projected - d).abs() < 0.05);
            }
            if t.length > 6000.0 {
                assert!(t.bounds.1.y - t.bounds.0.y > 100.0);
                assert!(
                    t.definition
                        .control_points
                        .iter()
                        .any(|p| p.bank_degrees != 0.0)
                );
            }
        }
    }
    #[test]
    fn ordered_physical_gates_allow_a_full_race_and_reject_shortcuts_reverse_and_verge_misses() {
        let track = Track::from_json(include_str!("../content/tracks/valley.json")).unwrap();
        let mut gates = GateProgress::default();
        let mut d = -1.0;
        while d < track.length + 1.0 {
            let next = (d + 5.0).min(track.length + 1.0);
            gates.observe(
                &track,
                track.sample(d, 0.0).0,
                track.sample(next, 0.0).0,
                d,
                next,
            );
            d = next;
        }
        assert_eq!(gates.completed_laps, 1);
        assert_eq!(gates.next, track.definition.checkpoints.len() - 1);
        let mut bad = GateProgress::default();
        let goal = track.length * track.definition.checkpoints[1].fraction;
        bad.observe(
            &track,
            track.sample(goal - 2.0, 25.0).0,
            track.sample(goal + 2.0, 25.0).0,
            goal - 2.0,
            goal + 2.0,
        );
        assert_eq!(bad.next, 1);
        bad.observe(
            &track,
            track.sample(goal + 2.0, 0.0).0,
            track.sample(goal - 2.0, 0.0).0,
            goal + 2.0,
            goal - 2.0,
        );
        assert_eq!(bad.next, 1);
        bad.observe(
            &track,
            track.sample(-1.0, 0.0).0,
            track.sample(track.length + 1.0, 0.0).0,
            -1.0,
            track.length + 1.0,
        );
        assert_eq!(bad.completed_laps, 0);
        bad.observe(
            &track,
            track.sample(goal - 2.0, 0.0).0,
            track.sample(goal + 2.0, 0.0).0,
            goal - 2.0,
            goal + 2.0,
        );
        assert_eq!(bad.next, 2);
    }
    #[test]
    fn spline_import_rejects_self_intersections() {
        let mut def = Track::default().definition;
        def.control_points.truncate(4);
        for (p, v) in def.control_points.iter_mut().zip([
            [0.0, 0.1, 0.0],
            [100.0, 0.1, 100.0],
            [0.0, 0.1, 100.0],
            [100.0, 0.1, 0.0],
        ]) {
            p.position = v;
        }
        assert!(Track::from_definition(def).is_err());
    }
    #[test]
    fn point_to_point_endpoints_never_wrap_and_grid_has_a_straight_approach() {
        let t = Track::from_json(include_str!("../content/tracks/valley.json")).unwrap();
        assert!(!t.definition.closed);
        let (start, q) = t.sample(0.0, 0.0);
        let (end, _) = t.sample(t.length, 0.0);
        assert!(start.distance(end) > 5000.0);
        assert!(t.sample(-20.0, 0.0).0.distance(start - q * Vec3::Z * 20.0) < 0.01);
        let progress = t.project(t.sample(t.length - 1.0, 0.0).0, 0.0).0;
        assert!((progress - (t.length - 1.0)).abs() < 0.05);
        assert!(t.sample(t.length + 5.0, 0.0).0.distance(end) < 5.01);
    }
}
