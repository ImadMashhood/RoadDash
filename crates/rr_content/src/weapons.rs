//! Typed arsenal authoring validated against shared explicit motion contact keys.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Weapons {
    pub version: u32,
    pub club: rr_core::weapons::Ability,
    #[serde(default = "legacy_ability")]
    pub chain: rr_core::weapons::Ability,
    #[serde(default = "legacy_ability")]
    pub whip: rr_core::weapons::Ability,
    #[serde(default = "legacy_ability")]
    pub rolling_pin: rr_core::weapons::Ability,
    #[serde(default = "legacy_ability")]
    pub baseball_bat: rr_core::weapons::Ability,
    #[serde(default = "default_armed_fraction")]
    pub starting_armed_fraction: f32,
}
/// Version-one catalogs had one ability. Preserve their behavior for absent families.
fn legacy_ability() -> rr_core::weapons::Ability {
    rr_core::weapons::Ability {
        source_contact: 0.32,
        cooldown: 0.85,
        stamina: 300.,
        lateral_reach: 2.55,
        longitudinal_reach: 0.9,
        pickup_radius: 2.5,
        direct_hits: 1,
    }
}
fn default_armed_fraction() -> f32 {
    0.5
}
impl Weapons {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let raw: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let value: Self = serde_json::from_value(raw.clone()).map_err(|e| e.to_string())?;
        if value.version == 2
            && rr_core::weapons::Weapon::ALL
                .iter()
                .any(|w| raw.get(w.id()).is_none())
        {
            return Err("schema2 requires every weapon family".into());
        }
        for weapon in rr_core::weapons::Weapon::ALL {
            let a = value.ability(weapon);
            if !matches!(value.version, 1 | 2)
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
                return Err("invalid weapon ability".into());
            }
        }
        Ok(value)
    }
    pub fn ability(&self, weapon: rr_core::weapons::Weapon) -> rr_core::weapons::Ability {
        use rr_core::weapons::Weapon;
        match weapon {
            Weapon::Club => self.club,
            Weapon::Chain => self.chain,
            Weapon::Whip => self.whip,
            Weapon::RollingPin => self.rolling_pin,
            Weapon::BaseballBat => self.baseball_bat,
        }
    }
    pub fn builtin() -> Self {
        Self::from_json(include_str!("../../../content/weapons.json")).expect("validated weapons")
    }
    pub fn validate_motion(&self, lib: &crate::motion::Library) -> Result<(), String> {
        for weapon in rr_core::weapons::Weapon::ALL {
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
                        - self.ability(weapon).source_contact)
                        .abs()
                        > 1e-6
                {
                    return Err("club contact timing differs from authored clip".into());
                }
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
    fn every_family_has_validated_distinct_cost_reach_and_stable_identity() {
        let data = Weapons::builtin();
        let mut costs = std::collections::BTreeSet::new();
        for weapon in rr_core::weapons::Weapon::ALL {
            let a = data.ability(weapon);
            assert!(costs.insert(a.stamina as u32));
            assert!(a.can_start(a.stamina));
            assert!(!a.can_start(a.stamina - 0.01));
            assert!(!a.reaches(a.lateral_reach, 0., 0., -1.));
            assert!(!a.reaches(a.lateral_reach, a.longitudinal_reach + 0.5, 0., 1.));
            assert!(a.reaches(a.lateral_reach, 0., 0., 1.));
            assert!(!a.reaches(a.lateral_reach + 0.5, 0., 0., 1.));
            let mut broken = serde_json::from_str::<serde_json::Value>(include_str!(
                "../../../content/weapons.json"
            ))
            .unwrap();
            broken[weapon.id()]["stamina"] = serde_json::json!(-1);
            assert!(Weapons::from_json(&broken.to_string()).is_err());
        }
    }
    #[test]
    fn old_club_only_catalog_remains_loadable_and_unknown_versions_fail() {
        let mut data = serde_json::from_str::<serde_json::Value>(include_str!(
            "../../../content/weapons.json"
        ))
        .unwrap();
        data["version"] = serde_json::json!(1);
        for name in ["chain", "whip", "rolling_pin", "baseball_bat"] {
            data.as_object_mut().unwrap().remove(name);
        }
        data["version"] = serde_json::json!(2);
        assert!(Weapons::from_json(&data.to_string()).is_err());
        data["version"] = serde_json::json!(1);
        let old = Weapons::from_json(&data.to_string()).unwrap();
        assert_eq!(old.whip.lateral_reach, old.club.lateral_reach);
        data["version"] = serde_json::json!(999);
        assert!(Weapons::from_json(&data.to_string()).is_err());
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
