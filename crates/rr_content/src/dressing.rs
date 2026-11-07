//! Semantic prop authoring; category and export coverage validated against the existing catalog.
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    Sidewalk,
    FieldEdge,
    Terrain,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Role {
    pub builder: String,
    pub size: [f32; 3],
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub role: String,
    pub along_m: f32,
    pub outward_m: f32,
    #[serde(default)]
    pub yaw_deg: f32,
}
fn one() -> u32 {
    1
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Zone {
    pub id: String,
    #[serde(default = "one")]
    pub budget_weight: u32,
    pub placement: Placement,
    pub from_m: f32,
    pub to_m: f32,
    pub spacing_m: f32,
    pub offset_m: f32,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub groups: Vec<String>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    pub max_props: usize,
    pub minimum_gap_m: f32,
    pub max_support_relief_m: f32,
    pub roles: BTreeMap<String, Role>,
    pub zones: Vec<Zone>,
    #[serde(default)]
    pub groups: BTreeMap<String, Vec<Member>>,
}
impl Config {
    pub fn from_json(s: &str, catalog: &crate::props::Catalog) -> Result<Self, String> {
        let c: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if c.schema_version != 1
            || !(1..=240).contains(&c.max_props)
            || !c.minimum_gap_m.is_finite()
            || !(0.5..=10.0).contains(&c.minimum_gap_m)
            || !c.max_support_relief_m.is_finite()
            || !(0.0..=0.6).contains(&c.max_support_relief_m)
            || c.zones.is_empty()
            || c.zones.len() > 32
            || c.zones.len() > c.max_props
            || c.groups.len() > 64
            || c.roles.len() > 64
        {
            return Err("invalid dressing budget".into());
        }
        for role in c.roles.values() {
            if !catalog.records.iter().any(|p| p.builder == role.builder)
                || role
                    .size
                    .iter()
                    .any(|v| !v.is_finite() || *v < 0.1 || *v > 20.0)
            {
                return Err("invalid semantic prop role".into());
            }
        }
        for members in c.groups.values() {
            if members.is_empty()
                || members.len() > 4
                || members.iter().any(|m| {
                    !c.roles.contains_key(&m.role)
                        || !m.along_m.is_finite()
                        || m.along_m.abs() > 12.0
                        || !m.outward_m.is_finite()
                        || !(0.0..=6.0).contains(&m.outward_m)
                        || !m.yaw_deg.is_finite()
                        || m.yaw_deg.abs() > 30.0
                })
            {
                return Err("invalid dressing group".into());
            }
        }
        let mut ids = std::collections::BTreeSet::new();
        for z in &c.zones {
            if z.id.is_empty()
                || !(1..=8).contains(&z.budget_weight)
                || !z.id.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                || !ids.insert(&z.id)
                || [z.from_m, z.to_m, z.spacing_m, z.offset_m]
                    .iter()
                    .any(|v| !v.is_finite())
                || z.from_m < 0.0
                || z.to_m <= z.from_m
                || z.to_m > 20000.0
                || z.spacing_m < 20.0
                || z.offset_m < 0.0
                || (z.roles.is_empty() && z.groups.is_empty())
                || z.groups.iter().any(|g| !c.groups.contains_key(g))
                || (z.placement == Placement::Sidewalk
                    && z.groups.iter().any(|g| {
                        c.groups.get(g).is_some_and(|ms| {
                            ms.iter()
                                .any(|m| m.outward_m > 0.1 || m.yaw_deg.abs() > 5.0)
                        })
                    }))
                || z.roles.iter().any(|r| !c.roles.contains_key(r))
            {
                return Err("invalid dressing zone".into());
            }
        }
        Ok(c)
    }
}
