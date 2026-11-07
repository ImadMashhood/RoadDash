//! Bounded presentation-only sky and lighting tuning; no simulation state.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema_version: u32,
    pub sky_resolution: u32,
    pub sky_nits: f32,
    pub environment_nits: f32,
    pub sun_lux: f32,
    pub ambient_nits: f32,
    pub exposure_ev100: f32,
    pub cloud_coverage: f32,
    pub wind_radians_per_second: f32,
    pub fog_start_m: f32,
    pub fog_end_m: f32,
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
            c.fog_start_m,
            c.fog_end_m,
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
            || c.fog_start_m < 50.0
            || c.fog_end_m < c.fog_start_m + 100.0
            || c.fog_end_m > 3000.0
        {
            return Err("invalid world art budget/tuning".into());
        }
        Ok(c)
    }
}
