//! Bounded presentation-only sky and lighting tuning; no simulation state.
use serde::Deserialize;
/// Only analytic operators supported by the pinned build; no LUT media required.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToneMapping {
    Reinhard,
    SomewhatBoringDisplayTransform,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    pub sky_resolution: u32,
    pub sky_nits: f32,
    pub environment_nits: f32,
    pub sun_lux: f32,
    pub sun_color_srgb: [f32; 3],
    pub ambient_color_srgb: [f32; 3],
    /// Direction from the world toward the sun (normalized by the renderer).
    pub sun_direction: [f32; 3],
    pub tonemapping: ToneMapping,
    pub ambient_nits: f32,
    pub exposure_ev100: f32,
    pub cloud_coverage: f32,
    /// Legacy authored wind retained for compatibility; stationary sky until
    /// independent cloud motion is implemented (never rotate sun/environment).
    pub wind_radians_per_second: f32,
    pub horizon_color_srgb: [f32; 3],
    /// Exponential-squared haze: bounded to retain >99% contrast at 100m.
    pub fog_density_per_m: f32,
}
impl Config {
    pub fn from_json(s: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        let values = [
            c.sky_nits,
            c.environment_nits,
            c.sun_lux,
            c.ambient_nits,
            c.exposure_ev100,
            c.cloud_coverage,
            c.wind_radians_per_second,
            c.fog_density_per_m,
        ];
        if c.schema_version != 1
            || !matches!(c.sky_resolution, 64 | 128 | 256 | 512)
            || values.iter().any(|v| !v.is_finite())
            || !(100.0..=40000.0).contains(&c.sky_nits)
            || !(0.0..=5000.0).contains(&c.environment_nits)
            || !(1000.0..=150000.0).contains(&c.sun_lux)
            || !(0.0..=8000.0).contains(&c.ambient_nits)
            || !(8.0..=16.0).contains(&c.exposure_ev100)
            || !(0.0..=1.0).contains(&c.cloud_coverage)
            || !(0.0..=0.01).contains(&c.wind_radians_per_second)
            || c.sun_color_srgb
                .iter()
                .chain(&c.ambient_color_srgb)
                .chain(&c.horizon_color_srgb)
                .any(|v| !v.is_finite() || !(0.05..=1.0).contains(v))
            || c.sun_direction.iter().any(|v| !v.is_finite())
            || !(0.2..=1.0).contains(&c.sun_direction[1])
            || !(0.25..=4.0).contains(&c.sun_direction.iter().map(|v| v * v).sum::<f32>())
            || !(0.0004..=0.001).contains(&c.fog_density_per_m)
        {
            return Err("invalid world art budget/tuning".into());
        }
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const JSON: &str = include_str!("../../../content/world/art.json");
    #[test]
    fn every_allowed_density_preserves_near_traffic_contrast() {
        let mut data: serde_json::Value = serde_json::from_str(JSON).unwrap();
        for step in 0..=60 {
            data["fog_density_per_m"] = serde_json::json!(0.0004 + step as f32 * 0.00001);
            let c = Config::from_json(&data.to_string()).unwrap();
            assert!((-(100.0 * c.fog_density_per_m).powi(2)).exp() > 0.99);
        }
    }
    #[test]
    fn atmosphere_rejects_opaque_near_field_and_invalid_authoring() {
        for (field, value) in [
            ("fog_density_per_m", serde_json::json!(-0.001)),
            ("fog_density_per_m", serde_json::json!(0.01)),
            ("fog_density_per_m", serde_json::json!(null)),
            ("horizon_color_srgb", serde_json::json!([0.68, 1.1, 0.82])),
            ("horizon_color_srgb", serde_json::json!([0.68, null, 0.82])),
        ] {
            let mut data: serde_json::Value = serde_json::from_str(JSON).unwrap();
            data[field] = value;
            assert!(Config::from_json(&data.to_string()).is_err(), "{field}");
        }
    }
    #[test]
    fn lighting_rejects_invalid_colors_direction_and_unsupported_lut_operator() {
        for (field, value) in [
            ("sun_color_srgb", serde_json::json!([1.0, -0.1, 0.5])),
            ("ambient_color_srgb", serde_json::json!([1.0, 2.0, 0.5])),
            ("sun_direction", serde_json::json!([0.0, 0.0, 0.0])),
            ("sun_direction", serde_json::json!([0.0, -1.0, 0.0])),
            ("tonemapping", serde_json::json!("ag_x")),
        ] {
            let mut data: serde_json::Value = serde_json::from_str(JSON).unwrap();
            data[field] = value;
            assert!(Config::from_json(&data.to_string()).is_err(), "{field}");
        }
        assert!(Config::from_json(JSON).is_ok());
    }
}
