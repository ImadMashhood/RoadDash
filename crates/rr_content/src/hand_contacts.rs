//! Visual glove shapes and attachment frames; never physical bones or hitboxes.
use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HandContacts {
    pub version: u32,
    pub seated_basis: [[f32; 4]; 2],
    pub hinges_m: [f32; 3],
    pub grip_angles_rad: [f32; 3],
    pub fist_angles_rad: [f32; 3],
    pub fist_keys: [Vec<u16>; 2],
    pub grip_sockets_m: [[f32; 3]; 2],
    pub weapon_rotation_xyzw: [[f32; 4]; 2],
}
impl HandContacts {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        if value.version != 1
            || value
                .hinges_m
                .iter()
                .any(|v| !v.is_finite() || !(0.01..=0.18).contains(v))
            || value.hinges_m.windows(2).any(|v| v[0] >= v[1])
            || value
                .grip_angles_rad
                .iter()
                .chain(&value.fist_angles_rad)
                .any(|v| !v.is_finite() || !(0.0..=1.5).contains(v))
            || value
                .grip_sockets_m
                .iter()
                .flatten()
                .any(|v| !v.is_finite() || v.abs() > 0.15)
            || value
                .seated_basis
                .iter()
                .chain(&value.weapon_rotation_xyzw)
                .any(|q| {
                    q.iter().any(|v| !v.is_finite())
                        || (q.iter().map(|v| v * v).sum::<f32>() - 1.0).abs() > 0.001
                })
            || value
                .fist_keys
                .iter()
                .any(|keys| keys.is_empty() || keys.len() > 64)
            || value.fist_keys[0]
                .iter()
                .any(|k| value.fist_keys[1].contains(k))
        {
            return Err("invalid visual hand contacts".into());
        }
        Ok(value)
    }
    pub fn validate_motion(&self, motion: &crate::motion::Library) -> Result<(), String> {
        for (side, keys) in self.fist_keys.iter().enumerate() {
            let mut unique = std::collections::HashSet::new();
            for id in keys {
                if !unique.insert(id)
                    || !motion
                        .keys
                        .iter()
                        .any(|k| k.id == *id && k.held & (1 << side) == 0)
                {
                    return Err("fist key missing, duplicated or still gripping".into());
                }
            }
        }
        Ok(())
    }
    /// Per-hand [grip, fist] weights; left/right contact bits are never swapped.
    pub fn shapes(&self, key: u16, held: u8) -> [[f32; 2]; 2] {
        std::array::from_fn(|side| {
            if self.fist_keys[side].contains(&key) {
                [0.0, 1.0]
            } else if held & (1 << side) != 0 {
                [1.0, 0.0]
            } else {
                [0.0, 0.0]
            }
        })
    }
}
/// Complete per-pose authoring state, independent from runtime clip selection.
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoringHand {
    pub grip: f32,
    pub fist: f32,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthoringHands {
    #[serde(rename = "L")]
    pub left: AuthoringHand,
    #[serde(rename = "R")]
    pub right: AuthoringHand,
}
impl AuthoringHands {
    pub fn validate(&self, held: &[String]) -> Result<(), String> {
        for (side, value) in [("L", &self.left), ("R", &self.right)] {
            if !value.grip.is_finite()
                || !value.fist.is_finite()
                || !(0.0..=1.0).contains(&value.grip)
                || !(0.0..=1.0).contains(&value.fist)
                || value.grip + value.fist > 1.0
                || (held.iter().any(|v| v == &format!("grip.{side}"))
                    && (value.grip != 1.0 || value.fist != 0.0))
            {
                return Err(format!("invalid authoring hand/contact state {side}"));
            }
        }
        Ok(())
    }
}
pub fn validate_authoring_assembly(
    json: &str,
    catalog: &crate::animation::Catalog,
) -> Result<(), String> {
    // The assembly also contains bike/prop/source records validated elsewhere.
    #[derive(Deserialize)]
    struct Entry {
        hand_values: AuthoringHands,
    }
    let entries: std::collections::BTreeMap<String, Entry> =
        serde_json::from_str(json).map_err(|e| e.to_string())?;
    for entry in entries.values() {
        entry.hand_values.validate(&[])?;
    }
    for pose in &catalog.poses {
        entries
            .get(&pose.id.to_string())
            .ok_or_else(|| format!("missing authoring hand state {}", pose.id))?
            .hand_values
            .validate(&pose.held_contacts)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn authoring_states_reject_busy_held_hands_and_out_of_range_weights() {
        for json in [
            r#"{"L":{"grip":1,"fist":1},"R":{"grip":0,"fist":0}}"#,
            r#"{"L":{"grip":-0.1,"fist":0},"R":{"grip":0,"fist":0}}"#,
            r#"{"L":{"grip":0,"fist":1},"R":{"grip":0,"fist":0}}"#,
        ] {
            let state: AuthoringHands = serde_json::from_str(json).unwrap();
            assert!(state.validate(&["grip.L".into()]).is_err());
        }
        assert!(serde_json::from_str::<AuthoringHands>(r#"{"L":{"grip":0,"fist":0}}"#).is_err());
    }
    #[test]
    fn current_context_authoring_directory_has_complete_valid_hands() {
        let root =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../art/animation/context");
        crate::animation::Catalog::validate_directory(&root.join("manifest.json")).unwrap();
    }
    const DATA: &str = include_str!("../../../content/hand-contacts.json");
    #[test]
    fn shapes_preserve_side_contacts_and_relaxed_hands() {
        let h = HandContacts::from_json(DATA).unwrap();
        assert_eq!(h.shapes(2020, 15), [[1., 0.], [1., 0.]]);
        assert_eq!(h.shapes(2221, 2), [[0., 1.], [1., 0.]]);
        assert_eq!(h.shapes(2231, 1), [[1., 0.], [0., 1.]]);
        assert_eq!(h.shapes(2364, 0), [[0., 0.], [0., 0.]]);
    }
    #[test]
    fn invalid_socket_hinge_and_quaternion_authoring_is_rejected() {
        for mutation in 0..5 {
            let mut v: serde_json::Value = serde_json::from_str(DATA).unwrap();
            match mutation {
                0 => v["hinges_m"][1] = 0.01.into(),
                1 => v["grip_sockets_m"][0][0] = 10.0.into(),
                2 => v["seated_basis"][0] = serde_json::json!([0, 0, 0, 0]),
                3 => v["fist_keys"][1] = v["fist_keys"][0].clone(),
                _ => v["physics"] = true.into(),
            }
            assert!(HandContacts::from_json(&v.to_string()).is_err());
        }
    }
}
