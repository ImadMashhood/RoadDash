use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Handling {
    pub id: String,
    pub dry_mass: f32,
    pub torque_nm: f32,
    pub top_speed: f32,
    pub nitro_speed: f32,
    pub grip: f32,
    pub turn_response: f32,
    pub spring: f32,
    pub damper: f32,
    pub drag: f32,
    pub road_assist: f32,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandlingCatalog {
    pub schema_version: u32,
    pub presets: Vec<Handling>,
}
impl HandlingCatalog {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let data: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        data.validate()?;
        Ok(data)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1 || self.presets.len() != 3 {
            return Err("schema 1 needs three handling presets".into());
        }
        let mut ids = std::collections::HashSet::new();
        for p in &self.presets {
            if !ids.insert(&p.id)
                || rr_core::ContentId::try_from(p.id.clone()).is_err()
                || ![
                    p.dry_mass,
                    p.torque_nm,
                    p.top_speed,
                    p.nitro_speed,
                    p.grip,
                    p.turn_response,
                    p.spring,
                    p.damper,
                    p.drag,
                    p.road_assist,
                ]
                .iter()
                .all(|v| v.is_finite())
                || !(150.0..=290.0).contains(&p.dry_mass)
                || !(20.0..=250.0).contains(&p.torque_nm)
                || !(25.0..=90.0).contains(&p.top_speed)
                || p.nitro_speed < p.top_speed
                || p.nitro_speed > 100.0
                || !(0.1..=3.0).contains(&p.grip)
                || !(0.4..=2.0).contains(&p.turn_response)
                || !(5000.0..=50000.0).contains(&p.spring)
                || !(500.0..=8000.0).contains(&p.damper)
                || !(0.01..=2.0).contains(&p.drag)
                || !(0.0..=1.0).contains(&p.road_assist)
            {
                return Err("invalid or duplicate handling preset".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn version_mass_and_nonfinite_presets_are_rejected() {
        let mut c =
            HandlingCatalog::from_json(include_str!("../../../content/handling.json")).unwrap();
        c.schema_version = 9;
        assert!(c.validate().is_err());
        c.schema_version = 1;
        c.presets[0].dry_mass = 0.0;
        assert!(c.validate().is_err());
        c.presets[0].dry_mass = 170.0;
        c.presets[0].grip = f32::NAN;
        assert!(c.validate().is_err());
    }
}
