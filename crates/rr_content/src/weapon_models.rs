//! Presentation-only arsenal. These IDs never change simulation weapon abilities.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub version: u32,
    pub license: String,
    pub records: Vec<Model>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Model {
    pub id: String,
    pub name: String,
    pub grip_origin: [f32; 3],
    pub axis: String,
    pub presentation_only: bool,
    pub lods: Vec<Lod>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lod {
    pub file: String,
    pub sha256: String,
    pub triangles: usize,
    pub bounds_y_up: [[f32; 3]; 2],
}
impl Catalog {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let expected = ["chain", "whip", "rolling_pin", "baseball_bat"];
        if value.version != 1 || value.license.is_empty() || value.records.len() != 4 {
            return Err("invalid weapon model catalog".into());
        }
        for id in expected {
            let entries: Vec<_> = value.records.iter().filter(|r| r.id == id).collect();
            if entries.len() != 1 {
                return Err("missing/duplicate weapon model".into());
            }
            let m = entries[0];
            if m.name.is_empty()
                || m.axis != "+Y"
                || m.grip_origin != [0.; 3]
                || !m.presentation_only
                || m.lods.len() != 3
            {
                return Err("invalid weapon grip contract".into());
            }
            let mut previous = usize::MAX;
            for (i, lod) in m.lods.iter().enumerate() {
                if lod.file != format!("weapons/{id}_lod{i}.glb")
                    || lod.sha256.len() != 64
                    || !lod.sha256.bytes().all(|b| b.is_ascii_hexdigit())
                    || lod.triangles == 0
                    || lod.triangles > 12000
                    || lod.triangles >= previous
                    || lod.bounds_y_up.iter().flatten().any(|v| !v.is_finite())
                    || (0..3).any(|a| lod.bounds_y_up[0][a] >= lod.bounds_y_up[1][a])
                    || lod.bounds_y_up[0][1] < -0.1
                    || lod.bounds_y_up[1][1] > 1.3
                {
                    return Err("invalid weapon LOD/bounds contract".into());
                }
                previous = lod.triangles;
            }
        }
        Ok(value)
    }
    pub fn builtin() -> Self {
        Self::from_json(include_str!("../../../assets/weapons/catalog.json"))
            .expect("valid original weapon models")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn arsenal_has_reducing_bounded_lods_and_exact_grips() {
        Catalog::builtin();
    }
    #[test]
    fn rejects_wrong_axis_unsafe_paths_and_missing_family() {
        let source = include_str!("../../../assets/weapons/catalog.json");
        for broken in [
            source.replace("+Y", "+Z"),
            source.replace("weapons/chain_lod0.glb", "../chain.glb"),
            source.replace("\"chain\"", "\"other\""),
        ] {
            assert!(Catalog::from_json(&broken).is_err());
        }
    }
}
