//! Original Hills route: shared logical stations keep both town streets race-equivalent.
use super::*;
use std::sync::OnceLock;
pub const JSON: &str = include_str!("../../content/tracks/hills.json");
pub const EAST_JSON: &str = include_str!("../../content/tracks/hills-east.json");
pub const LENGTH: f32 = 12_500.;
pub const CITY_END: f32 = 1_500.;
pub const SPLIT: f32 = 5_500.;
pub const MERGE: f32 = 7_400.;
pub const TOWN_END: f32 = 8_500.;
pub const BRIDGE_START: f32 = 4_620.;
pub const BRIDGE_END: f32 = 4_860.;
pub const BRIDGE_CENTER: f32 = (BRIDGE_START + BRIDGE_END) * 0.5;
impl Track {
    pub fn hills() -> Result<Self, String> {
        Self::from_json(JSON)
    }
    /// Cached source geometry; street sampling must never allocate a Track each frame.
    pub fn hills_city_source_track() -> &'static Self {
        static SOURCE: OnceLock<Track> = OnceLock::new();
        SOURCE.get_or_init(|| Self::metropolis().expect("validated city source"))
    }
    pub fn hills_city_source_origin() -> Vec3 {
        static ORIGIN: OnceLock<Vec3> = OnceLock::new();
        *ORIGIN.get_or_init(|| {
            Self::hills_city_source_track()
                .street_sample(0, 10_500., 0.)
                .0
        })
    }
    /// Imported city foundations and surfaces retain their original land profile.
    pub fn hills_city_ground_height(x: f32, z: f32) -> f32 {
        static PROFILE: OnceLock<rr_content::metropolis::ElevationProfile> = OnceLock::new();
        let profile = PROFILE.get_or_init(|| {
            Self::hills_city_source_track()
                .metropolis
                .as_ref()
                .unwrap()
                .elevation_profile
                .clone()
        });
        let origin = Self::hills_city_source_origin();
        profile.height(x + origin.x, z + origin.z) - origin.y + 8.
    }
    pub(super) fn hills_city_terrain(&self, x: f32, z: f32, station: f32, base: f32) -> f32 {
        if station >= 1500. {
            return base;
        }
        let t = ((station - 1000.) / 500.).clamp(0., 1.);
        let blend = t * t * (3. - 2. * t);
        Self::hills_city_ground_height(x, z).lerp(base, blend)
    }
    pub fn hills_city_to_local(source: Vec3) -> Vec3 {
        source - Self::hills_city_source_origin() + Vec3::Y * 8.
    }
    pub(super) fn import_hills_city_samples(&mut self) {
        let source = Self::hills_city_source_track();
        let origin = Self::hills_city_source_origin();
        // Preserve the curved city approach for the entire starting apron.
        let mut stations = vec![-300.];
        stations.extend(
            source.metropolis.as_ref().unwrap().traffic_paths[0]
                .progress
                .iter()
                .copied()
                .filter(|s| *s > 10_200. && *s < 11_500.)
                .map(|s| s - 10_500.),
        );
        stations.push(1000.);
        let mut points: Vec<_> = stations
            .iter()
            .map(|s| source.street_sample(0, *s + 10_500., 0.).0 - origin + Vec3::Y * 8.)
            .collect();
        let mut widths = vec![6.; points.len()];
        let mut banks = vec![0.; points.len()];
        let mut sectors = vec![Sector::City; points.len()];
        let mut surfaces = vec![Surface::Asphalt; points.len()];
        for (i, &s) in self.distances.iter().enumerate() {
            if s > 1000. {
                stations.push(s);
                points.push(self.points[i]);
                widths.push(self.widths[i]);
                banks.push(self.banks[i]);
                sectors.push(self.sectors[i]);
                surfaces.push(self.surfaces[i]);
            }
        }
        self.distances = stations;
        self.points = points;
        self.widths = widths;
        self.banks = banks;
        self.sectors = sectors;
        self.surfaces = surfaces;
        self.bounds = self.points.iter().fold(
            (Vec3::splat(f32::INFINITY), Vec3::splat(f32::NEG_INFINITY)),
            |(lo, hi), p| (lo.min(*p), hi.max(*p)),
        );
    }
    pub fn is_hills(&self) -> bool {
        self.definition.id.as_str() == "hills-01"
    }
    /// Canyon is a continuous world-space cut, independent of nearest hairpin.
    /// The road keeps its authored elevation and becomes a suspended deck.
    pub fn hills_canyon_weight(&self, x: f32, z: f32) -> f32 {
        if !matches!(self.definition.id.as_str(), "hills-01" | "hills-east-01") {
            return 0.;
        }
        let (center, rotation) = self.sample(BRIDGE_CENTER, 0.);
        let forward = (rotation * Vec3::Z).with_y(0.).normalize();
        let delta = Vec3::new(x - center.x, 0., z - center.z);
        let along = delta.dot(forward).abs();
        let across = delta.dot(Vec3::Y.cross(forward)).abs();
        let fade = |a: f32, b: f32, value: f32| {
            let t = ((value - a) / (b - a)).clamp(0., 1.);
            1. - t * t * (3. - 2. * t)
        };
        fade(70., 115., along) * fade(180., 480., across)
    }
    pub(super) fn hills_canyon_height(&self, x: f32, z: f32, base: f32) -> f32 {
        let weight = self.hills_canyon_weight(x, z);
        if weight == 0. {
            return base;
        }
        let (center, _) = self.sample(BRIDGE_CENTER, 0.);
        let bed = center.y - 82. + 2.5 * (x * 0.035).sin() * (z * 0.026).cos();
        base.lerp(bed.min(base - 0.1), weight)
    }
    pub(crate) fn hills_remote_height(&self, x: f32, z: f32) -> f32 {
        // Hills is authored monotonically northward. This longitudinal profile
        // gives the entire landscape one continuous datum, independent of which
        // hairpin is nearest in plan view. It never changes road geometry.
        let i = self
            .points
            .partition_point(|p| p.z <= z)
            .saturating_sub(1)
            .min(self.points.len() - 2);
        let a = self.points[i];
        let b = self.points[i + 1];
        let t = ((z - a.z) / (b.z - a.z)).clamp(0., 1.);
        let station = self.distances[i].lerp(self.distances[i + 1], t);
        let smooth = |a: f32, b: f32| {
            let t = ((station - a) / (b - a)).clamp(0., 1.);
            t * t * (3. - 2. * t)
        };
        let mountain = smooth(CITY_END, CITY_END + 300.) * (1. - smooth(SPLIT - 300., SPLIT));
        let dunes = smooth(TOWN_END, TOWN_END + 300.);
        a.y.lerp(b.y, t) - 0.35
            + mountain * (35. * (z * 0.009).sin() + 22. * (x * 0.012).cos())
            + dunes * (18. * (x * 0.013 + z * 0.006).sin() + 14. * (z * 0.017).sin())
    }
    pub(super) fn normalize_hills_stations(&mut self) {
        // Authoring knots are spaced at shared 50 m logical stations. The town
        // branches have equal symmetric detours and rejoin at the same station.
        let mut distances = Vec::with_capacity(self.points.len());
        let mut cursor = 0;
        for (i, pair) in self.definition.control_points.windows(2).enumerate() {
            let count = (Vec3::from_array(pair[0].position)
                .distance(Vec3::from_array(pair[1].position))
                / 1.25)
                .ceil() as usize;
            // Reparameterize each authored interval by its sampled arc length.
            // Sharper turns must not make equal logical station steps surge/slow
            // simply because Catmull-Rom parameter speed is nonuniform.
            let start = self.distances[cursor];
            let span = self.distances[cursor + count] - start;
            distances.extend(
                (0..count)
                    .map(|j| i as f32 * 50. + (self.distances[cursor + j] - start) / span * 50.),
            );
            cursor += count;
        }
        distances.push(LENGTH);
        self.distances = distances;
        self.length = LENGTH;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn city_opening_reuses_the_actual_final_race_kilometre_and_ground() {
        let hills = Track::hills().unwrap();
        let metro = Track::metropolis().unwrap();
        let origin = Track::hills_city_source_origin();
        for d in (0..=1000).step_by(10) {
            let expected = metro.street_sample(0, 10500. + d as f32, 0.).0 - origin + Vec3::Y * 8.;
            let (p, q) = hills.sample(d as f32, 0.);
            assert!(
                p.distance(expected) < 0.003,
                "city source drift at {d}: {p:?}/{expected:?}"
            );
            assert_eq!(hills.half_width(d as f32), 6.);
            if d < 900 {
                for lane in [-60., 0., 60.] {
                    let point = p - q * Vec3::X * lane;
                    let expected_ground =
                        metro.terrain_height(point.x + origin.x, point.z + origin.z) - origin.y
                            + 8.;
                    assert!(
                        (hills.terrain_height(point.x, point.z) - expected_ground).abs() < 0.005
                    );
                }
            }
        }
        let mut maximum_turn = 0_f32;
        for d in (1600..5380).step_by(5) {
            let a = (hills.sample(d as f32, 0.).1 * Vec3::Z)
                .with_y(0.)
                .normalize();
            let b = (hills.sample(d as f32 + 10., 0.).1 * Vec3::Z)
                .with_y(0.)
                .normalize();
            maximum_turn = maximum_turn.max(a.dot(b).clamp(-1., 1.).acos() / 10.);
        }
        assert!(
            maximum_turn > 0.009,
            "new hills must have genuinely tighter turns: {maximum_turn}"
        );
        // The canyon cut is deep but continuous, without moving the deck.
        let (p, q) = hills.sample(BRIDGE_CENTER, 0.);
        assert!(p.y - hills.terrain_height(p.x, p.z) > 75.);
        for along in -150..=150 {
            let a = p + (q * Vec3::Z).with_y(0.).normalize() * along as f32;
            let delta = hills.terrain_height(a.x + 0.01, a.z) - hills.terrain_height(a.x, a.z);
            assert!(
                delta.abs() < 0.15,
                "canyon must not introduce a height jump"
            );
        }
    }
    #[test]
    fn hills_has_strong_relief_flat_town_and_rejoins_before_town_exit() {
        let t = Track::hills().unwrap();
        assert_eq!(t.length, LENGTH);
        assert!(t.bounds.1.y - t.bounds.0.y > 160.);
        assert_eq!(t.definition.spawns.len(), 15);
        for s in [
            0., 1500., 5400., 5499., SPLIT, MERGE, 7401., 7800., TOWN_END, LENGTH,
        ] {
            assert!(
                t.sample_route(0, s, 0.)
                    .0
                    .distance(t.sample_route(1, s, 0.).0)
                    < 0.001
            );
        }
        assert!(
            t.sample_route(0, 6450., 0.)
                .0
                .distance(t.sample_route(1, 6450., 0.).0)
                > 129.
        );
        for s in (5600..8500).step_by(50) {
            for route in 0..2 {
                assert!((t.sample_route(route, s as f32, 0.).0.y - 70.).abs() < 0.001);
            }
        }
        // The fork/merge pavement overlaps: shared hazards remain visible there.
        assert!(t.same_corridor(0, 5550., 1, 5550.));
        assert!(t.same_corridor(0, 7350., 1, 7350.));
        assert!(!t.same_corridor(0, 6200., 1, 6200.));
        assert!(t.same_corridor(0, 7800., 1, 7800.));
    }
    #[test]
    fn both_hills_routes_project_and_complete_physical_gates() {
        let t = Track::hills().unwrap();
        for route in 0..2 {
            let mut g = GateProgress::default();
            for d in (0..12500).step_by(5) {
                let s = d as f32;
                let p = t.sample_route(route, s, 0.).0;
                let (projected, lane, _) = t.project_route(route, p, s);
                assert!(
                    (projected - s).abs() < 0.02,
                    "route {route} at {s}: {projected}"
                );
                assert!(lane.abs() < 0.02);
                g.observe_route(&t, route, p, t.sample_route(route, s + 5., 0.).0, s, s + 5.);
            }
            assert_eq!(g.completed_laps, 1);
            let index = t
                .definition
                .checkpoints
                .iter()
                .position(|g| (g.fraction * LENGTH - 6200.).abs() < 1.)
                .unwrap();
            let mut wrong = GateProgress {
                next: index,
                completed_laps: 0,
            };
            wrong.observe_route(
                &t,
                route,
                t.sample_route(1 - route, 6195., 0.).0,
                t.sample_route(1 - route, 6205., 0.).0,
                6195.,
                6205.,
            );
            assert_eq!(wrong.next, index);
            assert_eq!(
                t.choose_route(route, 6500., t.sample_route(1 - route, 6500., 0.).0, true),
                route
            );
        }
    }
    #[test]
    fn hills_distant_terrain_stays_continuous_across_nearest_road_changes() {
        let t = Track::hills().unwrap();
        assert!(t.points.windows(2).all(|p| p[1].z > p[0].z));
        let mut boundaries = 0;
        // Scan the mountain corridor for transitions between different bends;
        // evaluate tiny offsets around each candidate rather than only vertex counts.
        let z0 = t.sample(1800., 0.).0.z;
        let z1 = t.sample(5200., 0.).0.z;
        for zi in 0..((z1 - z0) / 16.) as usize {
            let z = z0 + zi as f32 * 16.;
            for xi in 0..((t.bounds.1.x - t.bounds.0.x + 400.) / 8.) as usize {
                let x = t.bounds.0.x - 200. + xi as f32 * 8.;
                let (sa, la) = t.nearby_station(x, z);
                let (sb, lb) = t.nearby_station(x + 8., z);
                if (sa - sb).abs() > 80.
                    && la.abs() > 65.
                    && lb.abs() > 65.
                    && t.hills_canyon_weight(x, z) == 0.
                    && t.hills_canyon_weight(x + 8., z) == 0.
                {
                    boundaries += 1;
                    let ha = t.terrain_height(x, z);
                    let hb = t.terrain_height(x + 8., z);
                    assert!(
                        (ha - hb).abs() < 4.,
                        "remote ridge at {x},{z}: {ha} -> {hb}"
                    );
                }
            }
        }
        assert!(
            boundaries > 10,
            "fixture must exercise actual bend boundaries"
        );
    }
    #[test]
    fn hills_grade_and_station_speed_are_bounded_and_scenery_clears_both_streets() {
        let t = Track::hills().unwrap();
        for route in 0..2 {
            for s in (0..12499).step_by(5) {
                let a = t.sample_route(route, s as f32, 0.).0;
                let b = t.sample_route(route, s as f32 + 1., 0.).0;
                assert!(
                    (0.96..1.04).contains(&a.distance(b)),
                    "{route} {s}: {}",
                    a.distance(b)
                );
                assert!((b.y - a.y).abs() / (b - a).with_y(0.).length() < 0.3);
            }
            assert!(!t.scenery_fits(t.sample_route(route, 6400., 0.).0, 3., 1.));
        }
    }
}
