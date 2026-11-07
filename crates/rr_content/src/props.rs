//! Visual prop intake only. No mass, collider or joint data may enter this contract.
use serde::Deserialize;
use std::collections::HashSet;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub records: Vec<Prop>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prop {
    pub id: String,
    pub name: String,
    pub group: String,
    pub builder: String,
    pub files: [String; 3],
    pub size: [f32; 3],
    pub center: [f32; 3],
    #[serde(default)]
    pub skin_files: Vec<String>,
    #[serde(default)]
    pub bones: usize,
}
impl Catalog {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let data: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        if data.schema_version != 1 || data.records.is_empty() || data.records.len() > 1024 {
            return Err("unsupported prop catalog".into());
        }
        let mut ids = HashSet::new();
        for p in &data.records {
            let safe = |s: &String| {
                !s.starts_with('/')
                    && !s.contains("..")
                    && !s.contains(':')
                    && !s.contains('\\')
                    && s.ends_with(".glb")
            };
            if !ids.insert(&p.id)
                || p.id.is_empty()
                || p.name.is_empty()
                || p.size
                    .iter()
                    .any(|v| !v.is_finite() || *v <= 0.0 || *v > 100.0)
                || p.center.iter().any(|v| !v.is_finite() || v.abs() > 100.0)
                || !p.files.iter().all(safe)
                || !p.skin_files.iter().all(safe)
                || (!p.skin_files.is_empty()
                    && (p.skin_files.len() != 2 || ![10, 11].contains(&p.bones)))
                || (p.skin_files.is_empty() && p.bones != 0)
            {
                return Err(format!("invalid visual prop {}", p.id));
            }
        }
        Ok(data)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complete_catalog_rejects_path_escapes_bad_bounds_and_unexpected_physics() {
        let json = include_str!("../../../content/props.json");
        let data = Catalog::from_json(json).unwrap();
        assert_eq!(data.records.len(), 369);
        for (old, new) in [
            ("models/", "../models/"),
            ("\"schema_version\":1", "\"schema_version\":9"),
        ] {
            assert!(Catalog::from_json(&json.replacen(old, new, 1)).is_err());
        }
        let mut value: serde_json::Value = serde_json::from_str(json).unwrap();
        value["records"][0]["size"][0] = serde_json::json!(0.0);
        assert!(Catalog::from_json(&value.to_string()).is_err());
        value["records"][0]["size"][0] = serde_json::json!(1.0);
        value["records"][0]["mass"] = serde_json::json!(80.0);
        assert!(Catalog::from_json(&value.to_string()).is_err());
    }
}
