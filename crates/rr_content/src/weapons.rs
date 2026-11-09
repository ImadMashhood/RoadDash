//! Club authoring validated against its explicit motion contact keys.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Weapons {
    pub version: u32,
    pub club: rr_core::weapons::Ability,
    #[serde(default = "default_armed_fraction")]
    pub starting_armed_fraction: f32,
}
fn default_armed_fraction() -> f32 {
    0.5
}
impl Weapons {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let a = value.club;
        if value.version != 1
            || !value.starting_armed_fraction.is_finite()
            || !(0.0..=1.0).contains(&value.starting_armed_fraction)
            || a.direct_hits != 1
            || [
                a.source_contact,
                a.cooldown,
                a.stamina,
                a.lateral_reach,
                a.longitudinal_reach,
                a.pickup_radius,
            ]
            .iter()
            .any(|v| !v.is_finite() || *v <= 0.0)
            || !(0.2..=0.8).contains(&a.source_contact)
            || a.cooldown < 0.65
            || a.cooldown > 3.0
            || a.stamina > 1000.0
            || !(1.9..=3.0).contains(&a.lateral_reach)
            || a.longitudinal_reach > 1.2
            || a.pickup_radius > 3.0
        {
            return Err("invalid club ability".into());
        }
        Ok(value)
    }
    pub fn builtin() -> Self {
        Self::from_json(include_str!("../../../content/weapons.json")).expect("validated weapons")
    }
    pub fn validate_motion(&self, lib: &crate::motion::Library) -> Result<(), String> {
        for (name, key) in [("club_left", 2162), ("club_right", 2172)] {
            let c = lib
                .clips
                .iter()
                .find(|c| c.name == name)
                .ok_or("missing club clip")?;
            if c.keys.get(2) != Some(&key)
                || (c
                    .durations
                    .get(..2)
                    .ok_or("missing club phases")?
                    .iter()
                    .sum::<f32>()
                    - self.club.source_contact)
                    .abs()
                    > 1e-6
            {
                return Err("club contact timing differs from authored clip".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shipped_ability_and_contact_keys_validate() {
        let a = Weapons::builtin();
        a.validate_motion(
            &crate::motion::Library::from_json(include_str!("../../../assets/models/motion.json"))
                .unwrap(),
        )
        .unwrap();
        assert!(a.club.reaches(2.4, 0.0, 0.0, 1.0));
        assert!(!a.club.reaches(a.club.lateral_reach + 0.5, 0.0, 0.0, 1.0));
        assert!(!a.club.reaches(-2.4, 0.0, 0.0, 1.0));
        assert!(!a.club.can_start(a.club.stamina - 0.01));
        assert!(a.club.can_start(a.club.stamina));
    }
    #[test]
    fn invalid_ability_is_rejected() {
        let text = include_str!("../../../content/weapons.json");
        for (a, b) in [
            (
                "\"starting_armed_fraction\": 0.5",
                "\"starting_armed_fraction\": 1.1",
            ),
            ("2.55", "99.0"),
            ("300.0", "-1.0"),
            ("0.32", "0.01"),
            ("0.85", "0.1"),
        ] {
            assert!(Weapons::from_json(&text.replace(a, b)).is_err());
        }
    }
    #[test]
    fn malformed_clip_returns_error_without_panicking() {
        let mut lib =
            crate::motion::Library::from_json(include_str!("../../../assets/models/motion.json"))
                .unwrap();
        lib.clips
            .iter_mut()
            .find(|c| c.name == "club_left")
            .unwrap()
            .durations
            .clear();
        assert!(Weapons::builtin().validate_motion(&lib).is_err());
    }
}
