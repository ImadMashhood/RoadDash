//! Approved race-first map: triangulated authoring data, independent of rendering.
use serde::Deserialize;
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
                    || p.half_width() > 5.
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
pub struct Metropolis {
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
}
impl Metropolis {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let m: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        m.validate()?;
        Ok(m)
    }
    pub fn validate(&self) -> Result<(), String> {
        self.track.validate()?;
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
                || b.style >= 12
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
        if self.street_props.len() > 1500 {
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
    fn approved_world_loads_and_rejects_broken_geometry_routes_and_styles() {
        let mut m =
            Metropolis::from_json(include_str!("../../../content/world/metropolis.json")).unwrap();
        assert_eq!(m.buildings.len(), 19068);
        assert_eq!(m.paths.len(), 2);
        m.buildings[0].style = 12;
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
        m.street_life.lane_sections[0].lane_width = 6.;
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
