//! Approved race-first map: triangulated authoring data, independent of rendering.
use serde::Deserialize;
/// One authoring source for roads, land, navigation and property support.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ElevationProfile {
    pub knots: Vec<[f32; 2]>,
    pub crossfall_amplitude: f32,
    pub crossfall_wavelength: f32,
}
impl ElevationProfile {
    pub fn height(&self, x: f32, z: f32) -> f32 {
        rr_core::elevation::height(
            &self.knots,
            [self.crossfall_amplitude, self.crossfall_wavelength],
            x,
            z,
        )
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.knots.len() < 2
            || self.knots.len() > 64
            || self
                .knots
                .iter()
                .flatten()
                .any(|v| !v.is_finite() || v.abs() > 20000.)
            || !self.crossfall_amplitude.is_finite()
            || self.crossfall_amplitude.abs() > 2.
            || !self.crossfall_wavelength.is_finite()
            || self.crossfall_wavelength < 500.
            || self.knots.windows(2).any(|p| {
                p[1][0] - p[0][0] < 300.
                    || 1.875 * (p[1][1] - p[0][1]).abs() / (p[1][0] - p[0][0]) > 0.04
            })
        {
            return Err("invalid elevation profile / grade".into());
        }
        Ok(())
    }
}
/// Ordered atlas/detail identity. Authoring palette parity is checked below.
pub const FACADE_FAMILIES: [&str; 24] = [
    "BUSS1", "BUSS4", "CHINA12", "CLSTGA11", "OFFC1", "OFFC2", "OFFC10", "RUNDWN11", "VICT11",
    "VICT12", "WHSE11", "WRHS1", "CHINA13", "CHINA14", "CLSTGA12", "CLSTGA13", "CLSTGA14",
    "OFFC11", "OFFC12", "OFFC13", "RUNDWN12", "VICT13", "VICT14", "WHSE12",
];
use std::collections::HashSet;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Shape {
    pub rings: Vec<Vec<[f32; 2]>>,
    pub triangles: Vec<[u32; 3]>,
}
impl Shape {
    pub fn vertices(&self) -> impl Iterator<Item = [f32; 2]> + '_ {
        self.rings.iter().flatten().copied()
    }
    fn triangle_area(&self) -> f64 {
        let v: Vec<_> = self.vertices().map(|p| p.map(f64::from)).collect();
        self.triangles
            .iter()
            .map(|t| {
                let [a, b, c] = t.map(|i| v[i as usize]);
                ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])).abs() * 0.5
            })
            .sum()
    }
    pub fn validate(&self) -> Result<(), String> {
        let v: Vec<_> = self.vertices().collect();
        if self.rings.is_empty()
            || self.rings.iter().any(|r| r.len() < 3)
            || v.len() > 4096
            || v.iter()
                .flatten()
                .any(|x| !x.is_finite() || x.abs() > 20000.)
            || self.triangles.is_empty()
        {
            return Err("invalid metropolis polygon".into());
        }
        for t in &self.triangles {
            if t.iter().any(|i| *i as usize >= v.len()) {
                return Err("metropolis triangle out of bounds".into());
            }
        }
        Ok(())
    }
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Path {
    pub points: Vec<[f32; 3]>,
    pub progress: Vec<f32>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Building {
    pub id: String,
    pub shape: Shape,
    pub height: f32,
    pub floors: u32,
    pub tier: u8,
    pub style: usize,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Surface {
    pub kind: u8,
    pub shape: Shape,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Street {
    pub id: String,
    pub points: Vec<[f32; 2]>,
    pub width_m: f32,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Junction {
    pub point: [f32; 2],
    pub street_id: String,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub patches: Vec<Shape>,
    pub id: String,
    pub shape: Shape,
    pub crop: String,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreetProp {
    pub id: String,
    pub kind: String,
    pub position: [f32; 2],
    pub heading: f32,
    pub size: [f32; 3],
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeightBand {
    pub weight: u32,
    pub min: u32,
    pub max: u32,
    pub downtown_extra: u32,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreetLife {
    pub schema_version: u32,
    pub main_traffic: usize,
    pub lane_sections: Vec<rr_core::streets::Section>,
    pub height_bands: Vec<HeightBand>,
    pub floor_height: f32,
}
impl StreetLife {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || !(48..=144).contains(&self.main_traffic)
            || self
                .height_bands
                .iter()
                .map(|b| u64::from(b.weight))
                .sum::<u64>()
                != 100
            || !self.floor_height.is_finite()
            || !(2.8..=3.5).contains(&self.floor_height)
            || self
                .height_bands
                .iter()
                .any(|b| b.min < 2 || b.max < b.min || b.max.saturating_add(b.downtown_extra) > 34)
            || self.lane_sections.len() > 64
        {
            return Err("invalid street-life budgets".into());
        }
        for route in 0..2 {
            let mut end = 0f32;
            let mut count = 0;
            for p in self.lane_sections.iter().filter(|p| p.route == route) {
                if !p.start.is_finite()
                    || !p.end.is_finite()
                    || !p.lane_width.is_finite()
                    || (p.start - end).abs() > 0.01
                    || p.end <= p.start
                    || !(1..=2).contains(&p.outbound)
                    || !(1..=2).contains(&p.inbound)
                    || !(2..=4).contains(&p.lanes())
                    || p.lane_width < 2.4
                    || p.half_width() > 6.
                {
                    return Err("invalid lane coverage or clearance".into());
                }
                end = p.end;
                count += 1;
            }
            if count == 0 || (end - 12000.).abs() > 0.01 {
                return Err("incomplete lane coverage".into());
            }
        }
        if self.lane_sections.iter().any(|p| p.route > 1) {
            return Err("invalid route lane profile".into());
        }
        Ok(())
    }
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParkedCar {
    pub id: String,
    pub position: [f32; 2],
    pub heading: f32,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pedestrian {
    #[serde(default)]
    pub crossing_end: Option<[f32; 2]>,
    pub id: String,
    pub position: [f32; 2],
    pub heading: f32,
    pub extent: f32,
    pub route: usize,
    pub station: f32,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CivicSite {
    pub id: String,
    pub kind: String,
    pub position: [f32; 2],
    pub heading: f32,
    pub size: [f32; 2],
    #[serde(default)]
    pub grounds: Vec<Shape>,
    #[serde(default)]
    pub paving: Vec<Shape>,
    #[serde(default)]
    pub entrances: Vec<[f32; 2]>,
    #[serde(default)]
    pub trees: Vec<[f32; 2]>,
    #[serde(default)]
    pub context_buildings: usize,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metropolis {
    #[serde(default)]
    pub civic_sites: Vec<CivicSite>,
    pub elevation_profile: ElevationProfile,
    pub schema_version: u32,
    pub source_layout_sha256: String,
    pub track: crate::track::TrackDef,
    pub paths: [Path; 2],
    pub traffic_paths: [Path; 2],
    pub bounds: [f32; 4],
    pub buildings: Vec<Building>,
    pub surfaces: Vec<Surface>,
    pub streets: Vec<Street>,
    pub junctions: Vec<Junction>,
    pub fields: Vec<Field>,
    pub street_props: Vec<StreetProp>,
    pub street_life: StreetLife,
    pub parked_cars: Vec<ParkedCar>,
    #[serde(default)]
    pub pedestrians: Vec<Pedestrian>,
}
impl Metropolis {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let m: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        m.validate()?;
        Ok(m)
    }
    pub fn validate(&self) -> Result<(), String> {
        self.elevation_profile.validate()?;
        let mut site_ids = HashSet::new();
        if self.civic_sites.iter().any(|s| {
            !site_ids.insert(&s.id)
                || !matches!(s.kind.as_str(), "city_hall" | "park" | "cafe")
                || !s.heading.is_finite()
                || s.position.iter().any(|v| !v.is_finite())
                || s.size
                    .iter()
                    .any(|v| !v.is_finite() || *v <= 0. || *v > 400.)
        }) {
            return Err("invalid civic site".into());
        }
        for site in &self.civic_sites {
            if site.grounds.len() + site.paving.len() > 2048 {
                return Err("civic grounds budget".into());
            }
            if site
                .entrances
                .iter()
                .chain(&site.trees)
                .flatten()
                .any(|v| !v.is_finite())
            {
                return Err("invalid civic positions".into());
            }
            for patch in site.grounds.iter().chain(&site.paving) {
                patch.validate()?;
            }
        }
        for field in &self.fields {
            if field.patches.is_empty() {
                return Err("missing field surface".into());
            }
            field.shape.validate()?;
            for patch in &field.patches {
                patch.validate()?;
            }
            let area = field.shape.triangle_area();
            let tiled: f64 = field.patches.iter().map(Shape::triangle_area).sum();
            if (tiled - area).abs() > area * 0.00001 + 0.02 {
                return Err("field patch area differs from boundary".into());
            }
        }
        for path in self.paths.iter().chain(self.traffic_paths.iter()) {
            for [x, y, z] in &path.points {
                if ![x, y, z].iter().all(|v| v.is_finite())
                    || (self.elevation_profile.height(*x, *z) - y).abs() > 0.002
                {
                    return Err("path elevation differs from authored land".into());
                }
            }
        }
        let mut walker_ids = HashSet::new();
        if self.pedestrians.len() > 480
            || self
                .pedestrians
                .iter()
                .filter(|p| p.crossing_end.is_some())
                .count()
                > 24
            || self.pedestrians.iter().any(|p| {
                p.crossing_end.is_some_and(|end| {
                    !end.iter().all(|v| v.is_finite())
                        || !(12. ..=26.)
                            .contains(&((end[0] - p.position[0]).hypot(end[1] - p.position[1])))
                }) || !walker_ids.insert(&p.id)
                    || p.id.is_empty()
                    || p.route > 1
                    || !p.station.is_finite()
                    || !(3150. ..12000.).contains(&p.station)
                    || !p.heading.is_finite()
                    || !p.extent.is_finite()
                    || !(4. ..=9.).contains(&p.extent)
                    || p.position
                        .iter()
                        .any(|v| !v.is_finite() || v.abs() > 20000.)
            })
        {
            return Err("invalid pedestrian patrol".into());
        }
        self.track.validate()?;
        // The race interval uses absolute route chainage; authoring keeps both
        // physical tails and the original grid offsets intact.
        let race = rr_core::race::RaceInterval::METROPOLIS;
        if self
            .track
            .checkpoints
            .iter()
            .skip(1)
            .take(self.track.checkpoints.len() - 2)
            .any(|g| !(race.start..race.finish).contains(&(g.fraction * 12000.)))
            || self
                .track
                .spawns
                .iter()
                .any(|s| !(0. ..race.start).contains(&race.grid_station(s.distance)))
        {
            return Err("metropolis race interval excludes route checks or grid".into());
        }
        self.street_life.validate()?;
        if self.parked_cars.len() > 300
            || self.parked_cars.iter().any(|p| {
                !p.heading.is_finite()
                    || p.position
                        .iter()
                        .any(|v| !v.is_finite() || v.abs() > 20000.)
            })
        {
            return Err("invalid parked traffic".into());
        }
        if self.schema_version != 1
            || self.track.id.as_str() != "metro-run-01"
            || self.source_layout_sha256.len() != 64
            || !self
                .source_layout_sha256
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
            || self.bounds[0] >= self.bounds[2]
            || self.bounds[1] >= self.bounds[3]
            || self.bounds.iter().any(|v| !v.is_finite())
            || self.buildings.len() > 25000
            || self.buildings.len() < 1000
        {
            return Err("invalid metropolis header/budget".into());
        }
        for p in self.paths.iter().chain(self.traffic_paths.iter()) {
            if p.points.len() != p.progress.len()
                || p.points.len() < 4
                || p.points.len() > 5000
                || p.progress[0] != 0.
                || *p.progress.last().unwrap() != 12000.
                || p.progress.windows(2).any(|w| w[1] <= w[0])
                || p.progress.iter().any(|x| !x.is_finite())
                || p.points.windows(2).any(|w| {
                    ((w[1][0] - w[0][0]).powi(2) + (w[1][2] - w[0][2]).powi(2)).sqrt() < 0.001
                })
                || p.points
                    .iter()
                    .flatten()
                    .any(|x| !x.is_finite() || x.abs() > 20000.)
            {
                return Err("invalid metropolis route".into());
            }
        }
        for (i, s) in self.paths[0].progress.iter().enumerate() {
            if (*s <= 3350. || *s >= 5362.5)
                && let Some(j) = self.paths[1]
                    .progress
                    .iter()
                    .position(|t| (*t - *s).abs() < 0.01)
                && self.paths[0].points[i] != self.paths[1].points[j]
            {
                return Err("shared route anchors differ".into());
            }
        }
        let mut ids = HashSet::new();
        for b in &self.buildings {
            b.shape.validate()?;
            if !ids.insert(&b.id)
                || !b.height.is_finite()
                || !(6. ..=110.).contains(&b.height)
                || b.floors < 2
                || b.tier > 3
                || b.style >= FACADE_FAMILIES.len()
            {
                return Err("invalid metropolis building".into());
            }
        }
        for s in &self.surfaces {
            s.shape.validate()?;
            if s.kind > 2 {
                return Err("unknown pavement".into());
            }
        }
        for f in &self.fields {
            f.shape.validate()?;
        }
        // ADR120: wider urban walks allow a bounded increase (1519 authored).
        if self.street_props.len() > 1600 {
            return Err("street furniture budget".into());
        }
        let mut prop_ids = HashSet::new();
        for p in &self.street_props {
            if !prop_ids.insert(&p.id)
                || ![
                    "street_lamp",
                    "broadleaf",
                    "flower_planter",
                    "bench",
                    "street_bin",
                    "hydrant",
                    "news_boxes",
                    "parking_meter",
                    "bus_sign",
                    "callbox",
                    "mailboxes",
                    "recycling_pair",
                    "utility_box",
                ]
                .contains(&p.kind.as_str())
                || !p.heading.is_finite()
                || p.position
                    .iter()
                    .any(|x| !x.is_finite() || x.abs() > 20000.)
                || p.size.iter().any(|v| !v.is_finite() || *v <= 0. || *v > 8.)
            {
                return Err("invalid street furniture".into());
            }
        }
        for j in &self.junctions {
            if !self.streets.iter().any(|s| s.id == j.street_id) {
                return Err("missing junction street".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authored_palette_order_matches_runtime_identity() {
        let palette: serde_json::Value = serde_json::from_str(include_str!(
            "../../../content/world/metropolis_facades.json"
        ))
        .unwrap();
        let families: Vec<_> = palette["families"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();
        assert_eq!(families, FACADE_FAMILIES);
        let provenance: serde_json::Value =
            serde_json::from_str(include_str!("../../../assets/metropolis/PROVENANCE.json"))
                .unwrap();
        assert_eq!(
            provenance["sources"].as_array().unwrap().len(),
            FACADE_FAMILIES.len() * 3
        );
        for (i, source) in provenance["sources"].as_array().unwrap().iter().enumerate() {
            assert_eq!(source["design"].as_str().unwrap(), FACADE_FAMILIES[i / 3]);
            assert_eq!(source["band"].as_u64().unwrap() as usize, i % 3);
        }
    }
    #[test]
    fn race_bounds_reject_internal_checks_outside_playable_interval() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        m.track.checkpoints[1].fraction = 250. / 12000.;
        assert!(m.validate().unwrap_err().contains("race interval"));
    }
    #[test]
    fn approved_world_loads_and_rejects_broken_geometry_routes_and_styles() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        assert_eq!(
            m.buildings
                .iter()
                .filter(|b| !b.id.starts_with("approach-") && !b.id.starts_with("civic-context-"))
                .count(),
            19068
        );
        assert_eq!(
            m.buildings
                .iter()
                .filter(|b| b.id.starts_with("approach-"))
                .count(),
            9
        );
        assert_eq!(
            m.buildings
                .iter()
                .filter(|b| b.id.starts_with("civic-context-"))
                .count(),
            31
        );
        assert_eq!(m.civic_sites.len(), 14);
        assert_eq!(m.paths.len(), 2);
        m.buildings[0].style = FACADE_FAMILIES.len();
        assert!(m.validate().is_err());
        m.buildings[0].style = 0;
        let old = m.paths[1].points[10];
        m.paths[1].points[10] = m.paths[1].points[9];
        assert!(m.validate().is_err());
        m.paths[1].points[10] = old;
        m.paths[1].progress[10] = f32::NAN;
        assert!(m.validate().is_err());
    }
    #[test]
    fn furniture_rejects_unknown_roles_duplicates_and_nonfinite_dimensions() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        assert!(m.street_props.len() > 200);
        let old = m.street_props[0].kind.clone();
        m.street_props[0].kind = "missing_asset".into();
        assert!(m.validate().is_err());
        m.street_props[0].kind = old;
        m.street_props[0].size[0] = f32::NAN;
        assert!(m.validate().is_err());
        m.street_props[0].size[0] = 0.5;
        m.street_props[1].id = m.street_props[0].id.clone();
        assert!(m.validate().is_err());
    }
    #[test]
    fn road_profiles_reject_gaps_too_many_lanes_and_overwide_layouts() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        m.street_life.lane_sections[0].start = 1.;
        assert!(m.validate().is_err());
        m.street_life.lane_sections[0].start = 0.;
        m.street_life.lane_sections[0].outbound = 3;
        assert!(m.validate().is_err());
        m.street_life.lane_sections[0].outbound = 1;
        m.street_life.lane_sections[0].lane_width = 7.;
        assert!(m.validate().is_err());
    }
    #[test]
    fn pedestrian_patrols_are_bounded_unique_and_finite() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        assert_eq!(m.pedestrians.len(), 480);
        assert_eq!(m.pedestrians.iter().filter(|p| p.route == 1).count(), 96);
        m.pedestrians[0].extent = 40.;
        assert!(m.validate().is_err());
        m.pedestrians[0].extent = 6.;
        m.pedestrians[1].id = m.pedestrians[0].id.clone();
        assert!(m.validate().is_err());
        m.pedestrians[1].id = "unique-pedestrian".into();
        m.pedestrians[1].position[0] = f32::NAN;
        assert!(m.validate().is_err());
    }
    #[test]
    fn crossing_endpoints_and_population_budgets_are_validated() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        assert_eq!(m.street_life.main_traffic, 144);
        assert_eq!(
            m.pedestrians
                .iter()
                .filter(|p| p.route == 1 && p.crossing_end.is_some())
                .count(),
            3
        );
        assert_eq!(
            m.pedestrians
                .iter()
                .filter(|p| p.crossing_end.is_some())
                .count(),
            13
        );
        let i = m
            .pedestrians
            .iter()
            .position(|p| p.crossing_end.is_some())
            .unwrap();
        let valid = m.pedestrians[i].crossing_end;
        m.pedestrians[i].crossing_end = Some(m.pedestrians[i].position);
        assert!(m.validate().is_err());
        m.pedestrians[i].crossing_end = Some([f32::NAN, 0.]);
        assert!(m.validate().is_err());
        m.pedestrians[i].crossing_end = valid;
        m.pedestrians.push(m.pedestrians[0].clone());
        assert!(m.validate().is_err());
    }
    #[test]
    fn polygon_indices_and_nonfinite_vertices_are_rejected() {
        let mut s = Shape {
            rings: vec![vec![[0., 0.], [2., 0.], [0., 2.]]],
            triangles: vec![[0, 1, 2]],
        };
        assert!(s.validate().is_ok());
        s.triangles[0][2] = 9;
        assert!(s.validate().is_err());
        s.triangles[0][2] = 2;
        s.rings[0][0][0] = f32::INFINITY;
        assert!(s.validate().is_err());
    }
}

#[cfg(test)]
mod elevation_tests {
    use super::*;
    #[test]
    fn reject_bad_profile_and_stale_path_heights() {
        let source = include_str!("../../../content/world/metropolis.json");
        let mut m = Metropolis::from_json(source).unwrap();
        m.paths[1].points[900][1] += 0.1;
        assert!(m.validate().unwrap_err().contains("path elevation"));
        let mut p = m.elevation_profile;
        p.knots[1][0] = p.knots[0][0];
        assert!(p.validate().is_err());
        p.knots[1][0] = f32::NAN;
        assert!(p.validate().is_err());
    }
}
