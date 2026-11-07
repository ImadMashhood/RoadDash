//! Authored mesh kit and archetype grammar validation; no engine or original game media.
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bounds {
    pub min: [f32; 3],
    pub max: [f32; 3],
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lod {
    pub lod: usize,
    pub sha256: String,
    pub source_sha256: String,
    pub triangles: [usize; 3],
    pub bounds: [Bounds; 3],
    pub bytes: usize,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Design {
    pub id: String,
    pub asset_type: String,
    pub masonry: String,
    pub width: f32,
    pub heights: [f32; 3],
    pub lods: Vec<Lod>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub records: Vec<Design>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Rules {
    pub schema_version: u32,
    pub max_active_buildings: usize,
    pub max_active_panels: usize,
    pub max_active_triangles: usize,
    pub detail_distance_m: f32,
    pub rules: BTreeMap<String, Vec<String>>,
}
impl Catalog {
    pub fn from_json(s: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if c.schema_version != 1 || c.records.is_empty() {
            return Err("invalid architecture catalog version/coverage".into());
        }
        let mut ids = BTreeSet::new();
        for d in &c.records {
            if d.id.is_empty()
                || d.masonry.is_empty()
                || !ids.insert(&d.id)
                || !d.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                || !matches!(d.asset_type.as_str(), "facade" | "structure")
                || !d
                    .masonry
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                || !d.width.is_finite()
                || d.width < 0.1
                || d.lods.len() != 3
                || d.heights.iter().any(|h| !h.is_finite() || *h < 0.0)
                || d.heights[0] < 0.1
                || d.heights[1] < 0.1
            {
                return Err(format!("invalid architecture design {}", d.id));
            }
            for (i, l) in d.lods.iter().enumerate() {
                if l.lod != i
                    || l.triangles[0] == 0
                    || l.bytes == 0
                    || [&l.sha256, &l.source_sha256]
                        .iter()
                        .any(|s| s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()))
                {
                    return Err("invalid kit export".into());
                }
                for b in &l.bounds {
                    for a in 0..3 {
                        if !b.min[a].is_finite() || !b.max[a].is_finite() || b.min[a] > b.max[a] {
                            return Err("invalid kit bounds".into());
                        }
                    }
                }
            }
        }
        Ok(c)
    }
}
impl Rules {
    pub fn from_json(s: &str, catalog: &Catalog) -> Result<Self, String> {
        let r: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if r.schema_version != 1
            || !(1..=32).contains(&r.max_active_buildings)
            || !(1..=1600).contains(&r.max_active_panels)
            || !(10000..=3000000).contains(&r.max_active_triangles)
            || !r.detail_distance_m.is_finite()
            || !(30.0..=220.0).contains(&r.detail_distance_m)
        {
            return Err("invalid building streaming budget".into());
        }
        for ids in r.rules.values() {
            if ids.is_empty()
                || ids
                    .iter()
                    .any(|id| !catalog.records.iter().any(|d| &d.id == id))
            {
                return Err("unknown or empty architecture pool".into());
            }
        }
        Ok(r)
    }
}
