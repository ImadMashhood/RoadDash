//! Baked original visual keyframes; deliberately excludes physical payloads.
use serde::Deserialize;
use std::collections::HashSet;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Key {
    pub held: u8,
    pub id: u16,
    pub root: [f32; 3],
    pub rotations: [[f32; 4]; 20],
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Clip {
    pub name: String,
    pub keys: Vec<u16>,
    pub durations: Vec<f32>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Limb {
    pub bit: u8,
    pub upper: usize,
    pub lower: usize,
    pub end: usize,
    pub offset: [f32; 3],
    pub target: [f32; 3],
    pub max_flex: f32,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Library {
    pub limbs: Vec<Limb>,
    pub version: u32,
    pub names: [String; 20],
    pub parents: [i8; 20],
    pub offsets: [[f32; 3]; 20],
    pub render_joints: [usize; 11],
    pub keys: Vec<Key>,
    pub clips: Vec<Clip>,
}
impl Library {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let lib: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        lib.validate()?;
        Ok(lib)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.limbs.len() != 4
            || self.limbs.iter().any(|l| {
                l.bit >= 4
                    || l.upper >= 20
                    || l.lower >= 20
                    || l.end >= 20
                    || self.parents[l.lower] != l.upper as i8
                    || self.parents[l.end] != l.lower as i8
                    || !l.max_flex.is_finite()
                    || !(1.0..=2.5).contains(&l.max_flex)
                    || l.offset
                        .iter()
                        .chain(l.target.iter())
                        .any(|v| !v.is_finite() || v.abs() > 5.0)
            })
        {
            return Err("invalid contact chain".into());
        }
        let mut ids = HashSet::new();
        let mut names = HashSet::new();
        if self.version != 1
            || self.keys.is_empty()
            || self.keys.len() > 512
            || self.clips.is_empty()
        {
            return Err("unsupported motion library".into());
        }
        for (i, &p) in self.parents.iter().enumerate() {
            if p >= i as i8
                || p < -1
                || (i == 0) != (p == -1)
                || !names.insert(&self.names[i])
                || self.offsets[i]
                    .iter()
                    .any(|v| !v.is_finite() || v.abs() > 5.0)
            {
                return Err("invalid motion hierarchy".into());
            }
        }
        if self.render_joints.iter().any(|&i| i >= 20) {
            return Err("invalid render map".into());
        }
        for k in &self.keys {
            if k.held > 15
                || !ids.insert(k.id)
                || k.root.iter().any(|v| !v.is_finite() || v.abs() > 10.0)
                || k.rotations.iter().any(|q| {
                    q.iter().any(|v| !v.is_finite())
                        || (q.iter().map(|v| v * v).sum::<f32>() - 1.0).abs() > 0.001
                })
            {
                return Err("invalid motion key".into());
            }
        }
        let mut clips = HashSet::new();
        for c in &self.clips {
            if c.name.is_empty()
                || !clips.insert(&c.name)
                || c.keys.is_empty()
                || c.keys.len() != c.durations.len()
                || c.keys.iter().any(|id| !ids.contains(id))
                || c.durations
                    .iter()
                    .any(|d| !d.is_finite() || *d <= 0.0 || *d > 5.0)
            {
                return Err("invalid motion clip".into());
            }
        }
        for id in [2020, 2030, 2040, 2900, 2901] {
            if !ids.contains(&id) {
                return Err("missing required riding key".into());
            }
        }
        for name in [
            "normal",
            "tuck",
            "accelerate",
            "turn_left",
            "turn_right",
            "punch_left",
            "punch_right",
            "kick_left",
            "kick_right",
            "pickup",
            "pickup_mirrored",
            "getup",
            "run",
        ] {
            if !clips.contains(&name.to_string()) {
                return Err("missing required motion clip".into());
            }
        }
        let bits: HashSet<_> = self.limbs.iter().map(|l| l.bit).collect();
        if bits.len() != 4 {
            return Err("duplicate contact bit".into());
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shipped_clips_are_valid_and_unknown_payloads_are_rejected() {
        let text = include_str!("../../../assets/models/motion.json");
        let lib = Library::from_json(text).unwrap();
        assert!(lib.clips.iter().any(|c| c.name == "pickup"));
        let mut value: serde_json::Value = serde_json::from_str(text).unwrap();
        value["physics"] = true.into();
        assert!(Library::from_json(&value.to_string()).is_err());
    }
    #[test]
    fn malformed_hierarchy_durations_and_quaternions_are_rejected() {
        let text = include_str!("../../../assets/models/motion.json");
        for mutation in 0..6 {
            let mut lib: Library = serde_json::from_str(text).unwrap();
            match mutation {
                0 => lib.parents[2] = 2,
                1 => lib.clips[0].durations[0] = 0.0,
                2 => lib.keys[0].rotations[0] = [0.0; 4],
                3 => lib.clips[0].name = "missing".into(),
                4 => lib.keys.retain(|k| k.id != 2020),
                _ => lib.limbs[1].bit = lib.limbs[0].bit,
            };
            assert!(lib.validate().is_err());
        }
    }
}
