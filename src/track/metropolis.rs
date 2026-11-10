use super::*;
use std::sync::{Arc, OnceLock};
pub const JSON: &str = include_str!("../../content/world/metropolis.json");
impl Track {
    pub fn metropolis() -> Result<Self, String> {
        static SOURCE: OnceLock<Arc<rr_content::metropolis::Metropolis>> = OnceLock::new();
        let data = if let Some(m) = SOURCE.get() {
            m.clone()
        } else {
            let m = Arc::new(rr_content::metropolis::Metropolis::from_json(JSON)?);
            let _ = SOURCE.set(m.clone());
            m
        };
        let mut track = Self::from_definition(data.track.clone())?;
        track.points = data.paths[0]
            .points
            .iter()
            .map(|p| Vec3::from_array(*p))
            .collect();
        track.distances = data.paths[0].progress.clone();
        track.widths = track.distances.iter().map(|_| 6.).collect();
        track.banks = vec![0.; track.points.len()];
        track.sectors = track
            .distances
            .iter()
            .map(|s| {
                if *s < 3150. {
                    Sector::Farms
                } else {
                    Sector::City
                }
            })
            .collect();
        track.surfaces = vec![Surface::Asphalt; track.points.len()];
        track.length = 12000.;
        let [x, z, xx, zz] = data.bounds;
        track.bounds = (Vec3::new(x, 0., z), Vec3::new(xx, 120., zz));
        track.metropolis = Some(data);
        Ok(track)
    }
    /// Lateral coordinate at known authoritative progress, without a full nearest-path search.
    pub fn lane_at(&self, route: usize, s: f32, p: Vec3) -> f32 {
        let (a, q) = self.sample_route(route, s, 0.);
        (p - a).dot(q * Vec3::NEG_X)
    }
    /// Carriageway centre follows the actual simplified street reservation; race geometry stays exact.
    pub fn street_offset(&self, route: usize, s: f32) -> f32 {
        if self.metropolis.is_none() {
            0.
        } else {
            self.lane_at(route, s, self.street_sample(route, s, 0.).0)
        }
    }
    pub fn street_sample(&self, route: usize, d: f32, lane: f32) -> (Vec3, Quat) {
        let Some(m) = &self.metropolis else {
            return self.sample_route(route, d, lane);
        };
        let p = &m.traffic_paths[route.min(1)];
        let s = d.clamp(0., self.length);
        let i = p
            .progress
            .partition_point(|x| *x <= s)
            .saturating_sub(1)
            .min(p.points.len() - 2);
        let t = (s - p.progress[i]) / (p.progress[i + 1] - p.progress[i]);
        let a = Vec3::from_array(p.points[i]);
        let b = Vec3::from_array(p.points[i + 1]);
        let f = (b - a).normalize();
        let side = Vec3::Y.cross(f).normalize();
        let q = Quat::from_mat3(&Mat3::from_cols(side, f.cross(side), f));
        let mut point = a.lerp(b, t) - side * lane;
        if d < 0. {
            point += f * d;
        } else if d > self.length {
            point += f * (d - self.length);
        }
        point.y = self.terrain_height(point.x, point.z) + 0.025;
        (point, q)
    }
    pub fn sample_route(&self, route: usize, d: f32, lane: f32) -> (Vec3, Quat) {
        let Some(m) = self.metropolis.as_ref().filter(|_| route == 1) else {
            return self.sample(d, lane);
        };
        let p = &m.paths[1];
        let s = d.clamp(0., self.length);
        let i = p
            .progress
            .partition_point(|x| *x <= s)
            .saturating_sub(1)
            .min(p.points.len() - 2);
        let t = (s - p.progress[i]) / (p.progress[i + 1] - p.progress[i]);
        let a = Vec3::from_array(p.points[i]);
        let b = Vec3::from_array(p.points[i + 1]);
        let f = (b - a).normalize();
        let side = Vec3::Y.cross(f).normalize();
        let q = Quat::from_mat3(&Mat3::from_cols(side, f.cross(side), f));
        let mut point = a.lerp(b, t) - q * Vec3::X * lane;
        if d < 0. {
            point += f * d
        } else if d > self.length {
            point += f * (d - self.length)
        }
        if self.metropolis.is_some() {
            point.y = self.terrain_height(point.x, point.z) + 0.025;
        } else if lane.abs() > self.half_width(d) + self.definition.verge_width {
            point.y = self.terrain_height(point.x, point.z) + 0.1;
        }
        (point, q)
    }
    pub fn project_route(&self, route: usize, point: Vec3, near: f32) -> (f32, f32, f32) {
        let Some(m) = self.metropolis.as_ref().filter(|_| route == 1) else {
            return self.project(point, near);
        };
        let path = &m.paths[1];
        let mut best = (0., 0., f32::INFINITY);
        for (i, pair) in path.points.windows(2).enumerate() {
            let a = Vec3::from_array(pair[0]);
            let b = Vec3::from_array(pair[1]);
            let edge = (b - a).with_y(0.);
            let t = ((point - a).with_y(0.).dot(edge) / edge.length_squared()).clamp(
                if i == 0 {
                    -self.start_apron / edge.length()
                } else {
                    0.
                },
                1.,
            );
            let delta = (point - a.lerp(b, t)).with_y(0.);
            let distance = delta.length();
            if distance < best.2 {
                best = (
                    path.progress[i] + t * (path.progress[i + 1] - path.progress[i]),
                    delta.dot(Vec3::new(-edge.z, 0., edge.x).normalize()),
                    distance,
                );
            }
        }
        best
    }
    pub fn same_corridor(&self, a: usize, sa: f32, b: usize, sb: f32) -> bool {
        self.metropolis.is_none() || rr_core::route::same_corridor(a, sa, b, sb)
    }
    pub fn choose_route(&self, current: usize, progress: f32, point: Vec3, player: bool) -> usize {
        if self.metropolis.is_none() {
            return 0;
        }
        rr_core::route::choice(
            current,
            progress,
            [
                self.project_route(0, point, progress).2,
                self.project_route(1, point, progress).2,
            ],
            player,
        )
    }
    pub fn curvature_route(&self, route: usize, d: f32) -> f32 {
        if route == 0 || self.metropolis.is_none() {
            return self.curvature(d);
        }
        let a = self.sample_route(route, d - 3., 0.).1 * Vec3::Z;
        let b = self.sample_route(route, d + 3., 0.).1 * Vec3::Z;
        a.cross(b).y.atan2(a.dot(b)) / 6.
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extended_start_grid_keeps_full_station_and_lane_on_both_routes() {
        let t = Track::metropolis().unwrap();
        for route in 0..2 {
            for spawn in &t.definition.spawns {
                let p = t.sample_route(route, spawn.distance, spawn.lane).0;
                let (d, l, _) = t.project_route(route, p, spawn.distance);
                assert!(
                    (d - spawn.distance).abs() < 0.05,
                    "{route}: {d} vs {}",
                    spawn.distance
                );
                assert!((l - spawn.lane).abs() < 0.05);
                assert!(spawn.lane.abs() + 0.6 < t.half_width(spawn.distance));
            }
        }
    }
    #[test]
    fn both_approved_routes_project_and_share_common_stations() {
        let t = Track::metropolis().unwrap();
        for route in 0..2 {
            for d in [0., 3100., 3350., 3900., 4500., 5000., 5362.5, 7000., 11990.] {
                let (p, _) = t.sample_route(route, d, 1.5);
                let (s, l, _) = t.project_route(route, p, d);
                assert!((s - d).abs() < 0.05, "{route} {d} {s}");
                assert!((l - 1.5).abs() < 0.05);
            }
        }
        assert!(
            t.sample_route(0, 4400., 0.)
                .0
                .distance(t.sample_route(1, 4400., 0.).0)
                > 200.
        );
        for d in [0., 3000., 6000., 12000.] {
            assert!(
                t.sample_route(0, d, 0.)
                    .0
                    .distance(t.sample_route(1, d, 0.).0)
                    < 0.01
            );
        }
    }
    #[test]
    fn street_flow_is_shared_after_merge_and_car_centres_follow_marked_lanes() {
        let t = Track::metropolis().unwrap();
        let m = t.metropolis.as_ref().unwrap();
        for d in [1000., 3000., 6000., 11000.] {
            assert!(
                t.street_sample(0, d, 0.)
                    .0
                    .distance(t.street_sample(1, d, 0.).0)
                    < 0.001
            );
        }
        for route in 0..2 {
            for d in [3300., 4100., 6500., 7800., 10000., 11000.] {
                let (c, q) = t.street_sample(route, d, 0.);
                for outgoing in [true, false] {
                    for slot in 0..2 {
                        let lane = rr_core::streets::lane_center(
                            &m.street_life.lane_sections,
                            route,
                            d,
                            outgoing,
                            slot,
                        );
                        let p = t.street_sample(route, d, lane).0;
                        assert!(((q.inverse() * (p - c)).x + lane).abs() < 0.01);
                        assert!(lane.abs() + 0.95 < t.half_width(d));
                    }
                }
            }
        }
    }
    #[test]
    fn playable_bounds_preserve_full_map_and_both_routes() {
        let t = Track::metropolis().unwrap();
        let m = t.metropolis.as_ref().unwrap();
        assert_eq!(t.length, 12000.);
        assert_eq!(t.points.len(), m.paths[0].points.len());
        assert_eq!(t.distances, m.paths[0].progress);
        assert_eq!(t.race_interval().length(), 11000.);
        for route in 0..2 {
            for spawn in &t.definition.spawns {
                let station = t.grid_station(spawn.distance);
                assert!((212. ..=492.).contains(&station));
                let p = t.sample_route(route, station, spawn.lane).0;
                let (d, lane, _) = t.project_route(route, p, station);
                assert!((d - station).abs() < 0.05);
                assert!((lane - spawn.lane).abs() < 0.05);
            }
            let mut g = GateProgress::default();
            for d in (500..11500).step_by(5) {
                let a = d as f32;
                g.observe_route(
                    &t,
                    route,
                    t.sample_route(route, a, 0.).0,
                    t.sample_route(route, a + 5., 0.).0,
                    a,
                    a + 5.,
                );
                assert_eq!(g.completed_laps, u32::from(d == 11495));
            }
            // Crossing the old physical endpoint cannot produce another result.
            g.observe_route(
                &t,
                route,
                t.sample_route(route, 11996., 0.).0,
                t.sample_route(route, 12000., 0.).0,
                11996.,
                12000.,
            );
            assert_eq!(g.completed_laps, 1);
            // Neither direct finish travel nor backwards crossing is a win.
            for (a, b) in [(11496., 11504.), (11504., 11496.), (500., 11504.)] {
                let mut fresh = GateProgress::default();
                fresh.observe_route(
                    &t,
                    route,
                    t.sample_route(route, a, 0.).0,
                    t.sample_route(route, b, 0.).0,
                    a,
                    b,
                );
                assert_eq!(fresh.completed_laps, 0);
            }
        }
    }
    #[test]
    fn branch_gates_require_the_selected_physical_corridor() {
        let t = Track::metropolis().unwrap();
        for route in 0..2 {
            let mut g = GateProgress::default();
            for d in (0..12000).step_by(8) {
                let a = d as f32;
                let b = (a + 8.).min(12000.);
                g.observe_route(
                    &t,
                    route,
                    t.sample_route(route, a, 0.).0,
                    t.sample_route(route, b, 0.).0,
                    a,
                    b,
                );
            }
            assert_eq!(g.completed_laps, 1);
        }
    }
}

#[cfg(test)]
mod gate_regressions {
    use super::*;
    #[test]
    fn legacy_maps_do_not_filter_targets_by_nonexistent_branches() {
        let legacy = Track::from_definition(
            rr_content::track::TrackDef::from_json(include_str!(
                "../../content/tracks/valley.json"
            ))
            .unwrap(),
        )
        .unwrap();
        assert!(legacy.same_corridor(0, 4400., 1, 4400.));
        let metro = Track::metropolis().unwrap();
        assert!(!metro.same_corridor(0, 4400., 1, 4400.));
        assert!(metro.same_corridor(0, 6000., 1, 6000.));
    }
    #[test]
    fn opposite_branch_and_large_shortcuts_cannot_award_gate_credit() {
        let t = Track::metropolis().unwrap();
        let index = t
            .definition
            .checkpoints
            .iter()
            .position(|g| (g.fraction * t.length - 4500.).abs() < 1.)
            .unwrap();
        for route in 0..2 {
            let mut g = GateProgress {
                next: index,
                completed_laps: 0,
            };
            g.observe_route(
                &t,
                route,
                t.sample_route(1 - route, 4496., 0.).0,
                t.sample_route(1 - route, 4504., 0.).0,
                4496.,
                4504.,
            );
            assert_eq!(g.next, index);
            g.observe_route(
                &t,
                route,
                t.sample_route(route, 4400., 0.).0,
                t.sample_route(route, 4600., 0.).0,
                4400.,
                4600.,
            );
            assert_eq!(g.next, index);
            g.observe_route(
                &t,
                route,
                t.sample_route(route, 4496., 0.).0,
                t.sample_route(route, 4504., 0.).0,
                4496.,
                4504.,
            );
            assert_eq!(g.next, index + 1);
        }
    }
}
