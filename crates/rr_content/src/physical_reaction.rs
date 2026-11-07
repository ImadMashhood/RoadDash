use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub max_speed: f32,
    pub max_spin: f32,
    pub attempt_seconds: f32,
    pub max_attempts: u8,
    pub work_joules: f32,
    pub protect_activation: f32,
    pub roll_activation: f32,
    pub slide_activation: f32,
}
impl Config {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if c.version != 1 || !(1..=3).contains(&c.max_attempts) {
            return Err("invalid reaction version/attempt count".into());
        }
        for (v, lo, hi) in [
            (c.max_speed, 3.0, 25.0),
            (c.max_spin, 1.0, 10.0),
            (c.attempt_seconds, 0.3, 2.0),
            (c.work_joules, 0.0, 250.0),
            (c.protect_activation, 0.0, 0.7),
            (c.roll_activation, 0.0, 0.5),
            (c.slide_activation, 0.0, 0.2),
        ] {
            if !v.is_finite() || !(lo..=hi).contains(&v) {
                return Err("invalid reaction parameter".into());
            }
        }
        Ok(c)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuning_rejects_unbounded_effort_and_attempts() {
        let valid = include_str!("../../../content/native_crash.json");
        assert!(Config::from_json(valid).is_ok());
        assert!(Config::from_json(&valid.replace("180.0", "18000.0")).is_err());
        assert!(
            Config::from_json(&valid.replace("\"max_attempts\": 2", "\"max_attempts\": 20"))
                .is_err()
        );
    }
}
