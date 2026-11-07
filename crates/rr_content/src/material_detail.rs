//! Provenance-linked micro-surface profiles for original authored albedo tiles.
use serde::Deserialize;
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    Brick,
    Wood,
    Stone,
    Metal,
    Plastic,
    Plaster,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub source: String,
    pub source_sha256: String,
    pub pixel_fnv64: String,
    pub kind: Kind,
    pub normal_strength: f32,
    pub roughness: f32,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    pub profiles: Vec<Profile>,
}
impl Config {
    pub fn from_json(s: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        let mut ids = std::collections::BTreeSet::new();
        if c.schema_version != 1 || c.profiles.is_empty() || c.profiles.len() > 64 {
            return Err("invalid material profile budget".into());
        }
        for p in &c.profiles {
            if p.source.contains("..")
                || !p.source.starts_with("art/buildings/mesh_assets/materials/")
                || !p.source.ends_with(".png")
                || p.source_sha256.len() != 64
                || !p.source_sha256.chars().all(|c| c.is_ascii_hexdigit())
                || p.pixel_fnv64.len() != 16
                || !p.pixel_fnv64.chars().all(|c| c.is_ascii_hexdigit())
                || !ids.insert(&p.pixel_fnv64)
                || !p.normal_strength.is_finite()
                || !(0.0..=0.6).contains(&p.normal_strength)
                || !p.roughness.is_finite()
                || !(0.1..=1.0).contains(&p.roughness)
            {
                return Err("invalid material profile/source".into());
            }
        }
        Ok(c)
    }
}
