//! Experimental Method B work, pose and rate limits; no dependency migration.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub policy: rr_core::velocity_muscles::Policy,
    pub frequency: f32,
    pub damping: f32,
    pub max_torque: f32,
    pub max_rate: f32,
    pub max_delta_rate: f32,
    pub guide_torque: f32,
    pub guide_rate: f32,
    pub support_normal_y: f32,
}
impl Config {
    pub fn from_json(s: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(s).map_err(|e| e.to_string())?;
        if c.version != 1 || !(1..=3).contains(&c.policy.max_attempts) {
            return Err("invalid velocity muscle version/attempts".into());
        }
        for (x, lo, hi) in [
            (c.frequency, 0.5, 15.0),
            (c.damping, 0.1, 3.0),
            (c.max_torque, 1.0, 40.0),
            (c.max_rate, 0.1, 6.0),
            (c.max_delta_rate, 0.1, 6.0),
            (c.guide_torque, 0.0, 120.0),
            (c.guide_rate, 0.0, 4.0),
            (c.support_normal_y, 0.55, 0.95),
            (c.policy.brace_seconds, 0.1, 1.0),
            (c.policy.roll_max_speed, 5.0, 30.0),
            (c.policy.roll_max_spin, 1.0, 12.0),
            (c.policy.attempt_seconds, 0.3, 2.0),
            (c.policy.retry_seconds, 0.2, 2.0),
        ] {
            if !x.is_finite() || !(lo..=hi).contains(&x) {
                return Err("invalid velocity muscle limit".into());
            }
        }
        if !c.policy.max_work_j.is_finite()
            || !(0.0..=200.0).contains(&c.policy.max_work_j)
            || !c.policy.power_w.is_finite()
            || !(0.0..=200.0).contains(&c.policy.power_w)
        {
            return Err("invalid muscle work budget".into());
        }
        Ok(c)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn configured_work_and_rates_are_bounded() {
        let text = include_str!("../../../content/velocity_muscles.json");
        assert!(Config::from_json(text).is_ok());
        let mut bad: serde_json::Value = serde_json::from_str(text).unwrap();
        bad["policy"]["max_work_j"] = 1000.0.into();
        assert!(Config::from_json(&bad.to_string()).is_err());
    }
}
