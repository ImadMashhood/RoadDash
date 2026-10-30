//! Versioned authoring contracts. Validation works headlessly without the game.
use rr_core::ContentId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
pub const CONTENT_SCHEMA_VERSION: u32 = 1;
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BikeDef {
    pub id: ContentId,
    pub display_name: String,
    pub top_speed_mps: f32,
    pub nitro_speed_mps: f32,
    pub acceleration_mps2: f32,
    pub nitro_acceleration_mps2: f32,
    pub brake_mps2: f32,
    pub coast_drag_mps2: f32,
    #[serde(default = "default_steering_base")]
    pub steering_rate_base: f32,
    #[serde(default = "default_steering_speed")]
    pub steering_rate_speed: f32,
    pub variants: Vec<CosmeticDef>,
}
fn default_steering_base() -> f32 {
    1.8
}
fn default_steering_speed() -> f32 {
    0.108
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CosmeticDef {
    pub id: ContentId,
    pub display_name: String,
    pub color_hex: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RiderDef {
    pub id: ContentId,
    pub display_name: String,
    pub bike_id: ContentId,
    pub palette_index: usize,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub bikes: Vec<BikeDef>,
    pub riders: Vec<RiderDef>,
    pub track_id: ContentId,
}
impl Catalog {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != CONTENT_SCHEMA_VERSION {
            return Err("unsupported content schema".into());
        }
        if self.bikes.is_empty() || self.riders.len() != 8 {
            return Err("prototype requires a bike and exactly eight riders".into());
        }
        let mut ids = HashSet::new();
        for bike in &self.bikes {
            if !ids.insert(bike.id.as_str()) {
                return Err("duplicate bike ID".into());
            }
            for v in [
                bike.top_speed_mps,
                bike.nitro_speed_mps,
                bike.acceleration_mps2,
                bike.nitro_acceleration_mps2,
                bike.brake_mps2,
                bike.coast_drag_mps2,
                bike.steering_rate_base,
                bike.steering_rate_speed,
            ] {
                if !v.is_finite() || v <= 0.0 {
                    return Err("bike tuning must be finite and positive".into());
                }
            }
            if bike.nitro_speed_mps < bike.top_speed_mps || bike.variants.len() < 2 {
                return Err("bike needs valid nitro tuning and two cosmetic variants".into());
            }
            let mut variants = HashSet::new();
            for variant in &bike.variants {
                if !variants.insert(variant.id.as_str())
                    || variant.color_hex.len() != 6
                    || !variant.color_hex.bytes().all(|v| v.is_ascii_hexdigit())
                {
                    return Err("invalid or duplicated cosmetic variant".into());
                }
            }
        }
        let mut rider_ids = HashSet::new();
        for rider in &self.riders {
            if !rider_ids.insert(rider.id.as_str())
                || rider.palette_index >= 8
                || !ids.contains(rider.bike_id.as_str())
            {
                return Err("invalid rider reference, palette, or duplicate ID".into());
            }
        }
        Ok(())
    }
    pub fn builtin() -> Self {
        Self::from_json(include_str!("../../../content/catalog.json"))
            .expect("built-in catalog must validate")
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shipped_catalog_and_references_validate() {
        Catalog::builtin().validate().unwrap();
    }
    #[test]
    fn earlier_schema_one_catalogs_gain_default_steering_without_losing_tuning() {
        let mut json: serde_json::Value =
            serde_json::from_str(include_str!("../../../content/catalog.json")).unwrap();
        let bike = json["bikes"][0].as_object_mut().unwrap();
        bike.remove("steering_rate_base");
        bike.remove("steering_rate_speed");
        let c = Catalog::from_json(&json.to_string()).unwrap();
        assert_eq!(c.bikes[0].steering_rate_base, 1.8);
        assert_eq!(c.bikes[0].top_speed_mps, 55.0);
    }
    #[test]
    fn invalid_tuning_and_schema_are_rejected() {
        let mut c = Catalog::builtin();
        c.bikes[0].brake_mps2 = f32::NAN;
        assert!(c.validate().is_err());
        c = Catalog::builtin();
        c.schema_version = 99;
        assert!(c.validate().is_err());
    }
    #[test]
    fn missing_references_and_duplicate_ids_are_rejected() {
        let mut c = Catalog::builtin();
        c.riders[0].bike_id = "missing".to_string().try_into().unwrap();
        assert!(c.validate().is_err());
        c = Catalog::builtin();
        c.riders[1].id = c.riders[0].id.clone();
        assert!(c.validate().is_err());
    }
}

pub mod track;
