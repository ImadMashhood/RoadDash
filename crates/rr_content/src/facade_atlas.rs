//! Provenance and bounds for facades projected from our authored 3D storeys.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    pub design: String,
    pub band: usize,
    pub source_sha256: String,
    pub bounds: super::architecture::Bounds,
    pub uv_min: [f32; 2],
    pub uv_max: [f32; 2],
    pub covered_pixels: usize,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mip {
    pub kind: String,
    pub level: u32,
    pub path: String,
    pub sha256: String,
    pub width: u32,
    pub height: u32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Atlas {
    pub schema_version: u32,
    pub width: u32,
    pub height: u32,
    pub tile_size: u32,
    pub gutter: u32,
    pub mip_levels: u32,
    pub source_manifest_sha256: String,
    pub generator_sha256: String,
    pub material_profiles_sha256: String,
    pub records: Vec<Tile>,
    pub images: Vec<Mip>,
}
fn digest(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|c| c.is_ascii_hexdigit())
}
impl Atlas {
    pub fn from_json(json: &str, catalog: &super::architecture::Catalog) -> Result<Self, String> {
        let a: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if a.schema_version != 1
            || a.width != 2048
            || a.height != 4096
            || a.tile_size != 128
            || a.gutter != 16
            || a.mip_levels != 5
            || !digest(&a.source_manifest_sha256)
            || !digest(&a.generator_sha256)
            || !digest(&a.material_profiles_sha256)
            || a.records.len() > 384
            || a.images.len() != 15
        {
            return Err("invalid facade atlas budget/provenance".into());
        }
        let mut seen = std::collections::BTreeSet::new();
        for tile in &a.records {
            let d = catalog
                .records
                .iter()
                .find(|d| d.id == tile.design)
                .ok_or("unknown atlas design")?;
            if tile.band > 2
                || !seen.insert((tile.design.as_str(), tile.band))
                || tile.source_sha256 != d.lods[2].sha256
                || tile.bounds.min != d.lods[2].bounds[tile.band].min
                || tile.bounds.max != d.lods[2].bounds[tile.band].max
                || tile.covered_pixels > 96 * 96
                || (0..2).any(|i| {
                    !tile.uv_min[i].is_finite()
                        || !tile.uv_max[i].is_finite()
                        || tile.uv_min[i] < 0.0
                        || tile.uv_max[i] > 1.0
                        || tile.uv_min[i] >= tile.uv_max[i]
                })
            {
                return Err("invalid facade tile/source/bounds".into());
            }
        }
        for d in &catalog.records {
            for band in 0..3 {
                if d.lods[2].triangles[band] > 0 && !seen.contains(&(d.id.as_str(), band)) {
                    return Err("missing authored facade band".into());
                }
            }
        }
        let mut mips = std::collections::BTreeSet::new();
        for m in &a.images {
            if !matches!(m.kind.as_str(), "albedo" | "normal" | "packed")
                || m.level >= a.mip_levels
                || !mips.insert((m.kind.as_str(), m.level))
                || m.path != format!("{}-mip{}.png", m.kind, m.level)
                || !digest(&m.sha256)
                || m.width != a.width >> m.level
                || m.height != a.height >> m.level
            {
                return Err("invalid facade mip chain".into());
            }
        }
        Ok(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn source() -> (String, super::super::architecture::Catalog) {
        (
            include_str!("../../../art/buildings/mesh_assets/far_facades/MANIFEST.json").into(),
            super::super::architecture::Catalog::from_json(include_str!(
                "../../../art/buildings/mesh_assets/storey_kit/MANIFEST.json"
            ))
            .unwrap(),
        )
    }
    #[test]
    fn every_authored_band_has_an_identity_preserving_tile() {
        let (s, c) = source();
        let a = Atlas::from_json(&s, &c).unwrap();
        assert_eq!(a.records.len(), 383);
        assert_eq!(
            a.records
                .iter()
                .map(|t| t.design.as_str())
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
            128
        );
        assert!(a.records.iter().all(|t| t.covered_pixels > 0));
    }
    #[test]
    fn incompatible_bounds_sources_mips_and_missing_tiles_are_rejected() {
        let (s, c) = source();
        for (ptr, value) in [
            (
                "/records/0/source_sha256",
                serde_json::json!("0".repeat(64)),
            ),
            ("/records/0/bounds/min/0", serde_json::json!(123.0)),
            ("/images/1/width", serde_json::json!(17)),
            ("/images/0/path", serde_json::json!("../arbitrary.png")),
            ("/mip_levels", serde_json::json!(8)),
            ("/material_profiles_sha256", serde_json::json!("invalid")),
        ] {
            let mut v: serde_json::Value = serde_json::from_str(&s).unwrap();
            *v.pointer_mut(ptr).unwrap() = value;
            assert!(Atlas::from_json(&v.to_string(), &c).is_err(), "{ptr}");
        }
        let mut v: serde_json::Value = serde_json::from_str(&s).unwrap();
        v["records"].as_array_mut().unwrap().remove(0);
        assert!(Atlas::from_json(&v.to_string(), &c).is_err());
    }
}
