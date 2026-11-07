use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub roll_min_speed: f32,
    pub roll_min_age: f32,
    pub roll_rate: f32,
    pub roll_energy_fraction: f32,
    pub release_seconds: f32,
    pub release_activation: f32,
    pub hip_back: f32,
    pub knee_back: f32,
}
impl Config {
    pub fn from_json(s: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if c.version != 1 {
            return Err("unknown crash motion version".into());
        }
        for (x, lo, hi) in [
            (c.roll_min_speed, 2.0, 10.0),
            (c.roll_min_age, 0.15, 1.0),
            (c.roll_rate, 0.1, 3.0),
            (c.roll_energy_fraction, 0.0, 0.1),
            (c.release_seconds, 0.15, 0.6),
            (c.release_activation, 0.0, 0.5),
            (c.hip_back, 0.0, 0.4),
            (c.knee_back, 0.0, 1.0),
        ] {
            if !x.is_finite() || !(lo..=hi).contains(&x) {
                return Err("invalid crash motion tuning".into());
            }
        }
        Ok(c)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_energy_and_joint_targets() {
        let s = include_str!("../../../content/crash_motion.json");
        assert!(Config::from_json(s).is_ok());
        for k in [
            "roll_energy_fraction",
            "hip_back",
            "release_activation",
            "roll_rate",
        ] {
            let mut v: serde_json::Value = serde_json::from_str(s).unwrap();
            v[k] = 100.0.into();
            assert!(Config::from_json(&v.to_string()).is_err());
        }
    }
}
