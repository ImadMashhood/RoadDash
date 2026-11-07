use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub auto_seek: bool,
    pub brace_activation: f32,
    pub neutral_delay: f32,
    pub walk_speed: f32,
    pub stride_length: f32,
    pub mount_radius: f32,
    pub mount_height: f32,
    pub mount_speed: f32,
    pub mount_facing: f32,
    pub rescue_after: f32,
    pub rescue_penalty: f32,
}
impl Config {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        for (value, min, max) in [
            (c.brace_activation, 0.0, 0.2),
            (c.neutral_delay, 0.25, 0.5),
            (c.walk_speed, 1.0, 8.0),
            (c.stride_length, 1.0, 5.0),
            (c.mount_radius, 0.3, 1.2),
            (c.mount_height, 0.3, 1.5),
            (c.mount_speed, 0.1, 3.0),
            (c.mount_facing, 0.0, 0.95),
            (c.rescue_after, 10.0, 180.0),
            (c.rescue_penalty, 0.0, 30.0),
        ] {
            if !value.is_finite() || !(min..=max).contains(&value) {
                return Err("invalid recovery tuning".into());
            }
        }
        if c.version != 1 {
            return Err("unknown recovery version".into());
        }
        Ok(c)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validated_tuning_rejects_teleport_and_immediate_assist_settings() {
        let json = include_str!("../../../content/recovery.json");
        assert!(Config::from_json(json).is_ok());
        for (key, value) in [
            ("neutral_delay", 0.0),
            ("walk_speed", 100.0),
            ("mount_radius", 12.0),
            ("version", 2.0),
        ] {
            let mut d: serde_json::Value = serde_json::from_str(json).unwrap();
            d[key] = value.into();
            assert!(Config::from_json(&d.to_string()).is_err());
        }
    }
}
