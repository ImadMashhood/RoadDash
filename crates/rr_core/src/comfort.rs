//! Device-independent comfort and normalized input rules.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Comfort {
    pub fov_degrees: f32,
    pub smoothing: f32,
    pub shake: f32,
    pub speed_fov: bool,
    pub anti_nausea: bool,
    pub speed_lines: bool,
    pub deadzone: f32,
}
impl Default for Comfort {
    fn default() -> Self {
        Self {
            fov_degrees: 75.0,
            smoothing: 10.0,
            shake: 0.5,
            speed_fov: true,
            anti_nausea: false,
            speed_lines: false,
            deadzone: 0.15,
        }
    }
}
impl Comfort {
    pub fn validate(&self) -> Result<(), String> {
        if !self.fov_degrees.is_finite()
            || !(60.0..=95.0).contains(&self.fov_degrees)
            || !self.smoothing.is_finite()
            || !(2.0..=20.0).contains(&self.smoothing)
            || !self.shake.is_finite()
            || !(0.0..=1.0).contains(&self.shake)
            || !self.deadzone.is_finite()
            || !(0.05..=0.4).contains(&self.deadzone)
        {
            return Err("comfort value outside supported range".into());
        }
        Ok(())
    }
    pub fn fov(&self, speed: f32, recovering: bool) -> f32 {
        (self.fov_degrees
            + if self.speed_fov && !self.anti_nausea && !recovering {
                (speed.max(0.0) * 0.10).min(8.0)
            } else {
                0.0
            })
        .clamp(60.0, 95.0)
        .to_radians()
    }
}
pub fn axis(value: f32, deadzone: f32) -> f32 {
    if !value.is_finite() {
        return 0.0;
    }
    let value = value.clamp(-1.0, 1.0);
    value.signum() * ((value.abs() - deadzone) / (1.0 - deadzone)).clamp(0.0, 1.0)
}
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum InputContext {
    #[default]
    Menu,
    Riding,
    OnFoot,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deadzone_is_continuous_and_finite() {
        for v in [-0.15, 0.0, 0.15, f32::NAN] {
            assert_eq!(axis(v, 0.15), 0.0);
        }
        assert_eq!(axis(1.0, 0.15), 1.0);
        assert_eq!(axis(-1.0, 0.15), -1.0);
        assert!(axis(0.16, 0.15) < 0.02);
    }
    #[test]
    fn comfort_limits_and_anti_nausea_are_explicit() {
        let mut c = Comfort::default();
        c.validate().unwrap();
        assert!(c.fov(80.0, false) > c.fov(0.0, false));
        c.anti_nausea = true;
        assert_eq!(c.fov(80.0, false), c.fov(0.0, false));
        c.fov_degrees = 96.0;
        assert!(c.validate().is_err());
        c.fov_degrees = f32::NAN;
        assert!(c.validate().is_err());
    }
}
