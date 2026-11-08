//! Authoring validation for all currently placed sidewalk prop types.
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    Topple,
    Crush,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub durability_j: f32,
    pub mass_kg: f32,
    pub style: Style,
    pub soft: bool,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    pub min_speed_mps: f32,
    pub cooldown_s: f32,
    pub max_dynamic_bodies: usize,
    pub settle_after_s: f32,
    pub profiles: BTreeMap<String, Profile>,
}
impl Config {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<(), String> {
        let keys = [
            "hydrant",
            "street_lamp",
            "bench",
            "street_bin",
            "flower_planter",
            "broadleaf",
            "parking_meter",
            "bus_sign",
            "callbox",
            "mailboxes",
            "recycling_pair",
            "utility_box",
            "news_boxes",
        ];
        if self.schema_version != 1
            || !(1..=96).contains(&self.max_dynamic_bodies)
            || !self.min_speed_mps.is_finite()
            || !(1.0..=10.).contains(&self.min_speed_mps)
            || !self.cooldown_s.is_finite()
            || !(0.1..=2.).contains(&self.cooldown_s)
            || !self.settle_after_s.is_finite()
            || !(2.0..=20.).contains(&self.settle_after_s)
            || self.profiles.len() != keys.len()
            || keys.iter().any(|k| !self.profiles.contains_key(*k))
        {
            return Err("invalid destruction configuration/coverage".into());
        }
        for p in self.profiles.values() {
            if !p.durability_j.is_finite()
                || !(10.0..=100000.).contains(&p.durability_j)
                || !p.mass_kg.is_finite()
                || !(1.0..=150.).contains(&p.mass_kg)
            {
                return Err("invalid prop strength/mass".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_roles_are_validated_and_unbounded_budgets_rejected() {
        let mut c =
            Config::from_json(include_str!("../../../content/world/destruction.json")).unwrap();
        assert_eq!(c.profiles.len(), 13);
        c.max_dynamic_bodies = 99999;
        assert!(c.validate().is_err());
        c.max_dynamic_bodies = 48;
        c.profiles.remove("bench");
        assert!(c.validate().is_err());
    }
}
