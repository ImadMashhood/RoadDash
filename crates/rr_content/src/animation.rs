//! Authoring-only static keyframes. No Bevy, animation playback or physics payload.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::path::{Component, Path};

type Matrix = [[f32; 4]; 4];
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub schema_version: u32,
    pub pose_kind: String,
    pub runtime_connected: bool,
    pub physics_changed: bool,
    pub poses: Vec<Descriptor>,
    pub use_cases: Vec<UseCase>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Descriptor {
    pub id: u16,
    pub name: String,
    pub actor: String,
    pub family: String,
    pub source_slots: Vec<u16>,
    pub review: String,
    pub held_contacts: Vec<String>,
    pub screenshot: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UseCase {
    pub id: String,
    pub family: String,
    pub actor: String,
    pub state: String,
    pub pose_ids: Vec<u16>,
    pub entry_pose: u16,
    pub exit_pose: u16,
    pub phase_labels: Vec<String>,
    pub interrupts: Vec<String>,
    pub timing_basis: String,
    pub notes: String,
    pub review: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Bind {
    pub parent: Option<String>,
    pub head: [f32; 3],
    pub tail: [f32; 3],
    pub matrix_local: Matrix,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pose {
    pub schema_version: u32,
    pub id: u16,
    pub frame_id: u16,
    pub name: String,
    pub actor: String,
    pub family: String,
    pub source_slots: Vec<u16>,
    pub interpretation: String,
    pub review: String,
    pub held_contacts: Vec<String>,
    pub released_contacts: Vec<String>,
    pub contact_errors_m: BTreeMap<String, f32>,
    pub reference_confidence: String,
    pub root_matrix: Matrix,
    pub rig_matrix: Matrix,
    pub bone_basis: BTreeMap<String, Matrix>,
    pub bone_world: BTreeMap<String, Matrix>,
    pub bind: BTreeMap<String, Bind>,
    pub max_attachment_error_m: f32,
    pub max_bone_length_error_m: f32,
    pub screenshot: String,
}
fn safe_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\\')
        && Path::new(path)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
}
fn rigid(m: &Matrix) -> bool {
    if m.iter()
        .flatten()
        .any(|v| !v.is_finite() || v.abs() > 100.0)
        || m[3] != [0.0, 0.0, 0.0, 1.0]
    {
        return false;
    }
    for a in 0..3 {
        for b in 0..3 {
            let dot: f32 = (0..3).map(|i| m[i][a] * m[i][b]).sum();
            if (dot - if a == b { 1.0 } else { 0.0 }).abs() > 0.002 {
                return false;
            }
        }
    }
    let determinant = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    (determinant - 1.0).abs() < 0.003
}
fn point(m: &Matrix, v: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| (0..3).map(|j| m[i][j] * v[j]).sum::<f32>() + m[i][3])
}
fn inverse_point(m: &Matrix, v: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| (0..3).map(|j| m[j][i] * (v[j] - m[j][3])).sum())
}
fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    a.iter()
        .zip(b)
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f32>()
        .sqrt()
}
fn valid_actor(actor: &str) -> bool {
    matches!(actor, "human" | "cow")
}
fn contacts(values: &[String]) -> bool {
    let mut seen = HashSet::new();
    values.iter().all(|v| {
        matches!(
            v.as_str(),
            "seat"
                | "grip.L"
                | "grip.R"
                | "peg.L"
                | "peg.R"
                | "ground.L"
                | "ground.R"
                | "ground.front.L"
                | "ground.front.R"
                | "ground.hind.L"
                | "ground.hind.R"
        ) && seen.insert(v)
    })
}
impl Catalog {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let catalog: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        catalog.validate()?;
        Ok(catalog)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.pose_kind != "static-keyframes"
            || self.runtime_connected
            || self.physics_changed
            || self.poses.is_empty()
            || self.poses.len() > 4096
            || self.use_cases.is_empty()
        {
            return Err("unsupported or non-static authoring catalog".into());
        }
        let mut ids = HashSet::new();
        for p in &self.poses {
            if !ids.insert(p.id)
                || p.name.is_empty()
                || !valid_actor(&p.actor)
                || !contacts(&p.held_contacts)
                || !safe_path(&p.screenshot)
                || p.source_slots.iter().any(|s| *s >= 326)
            {
                return Err(format!("invalid or duplicate descriptor {}", p.id));
            }
        }
        let mut cases = HashSet::new();
        for c in &self.use_cases {
            if !cases.insert(&c.id)
                || c.pose_ids.first() != Some(&c.entry_pose)
                || c.pose_ids.last() != Some(&c.exit_pose)
                || c.pose_ids.iter().any(|id| !ids.contains(id))
                || !valid_actor(&c.actor)
                || c.notes.is_empty()
                || c.phase_labels.is_empty()
                || c.timing_basis != "unassigned"
            {
                return Err(format!("invalid use case {} or guessed timing", c.id));
            }
            for id in &c.pose_ids {
                if self.poses.iter().find(|p| p.id == *id).unwrap().actor != c.actor {
                    return Err(format!("mixed skeletons in {}", c.id));
                }
            }
        }
        Ok(())
    }
    /// Validate all editable pose files, independent of the renderer/asset server.
    pub fn validate_directory(path: &Path) -> Result<Self, String> {
        let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        let catalog = Self::from_json(&text)?;
        let folder = path.parent().unwrap_or(Path::new("."));
        for d in &catalog.poses {
            let file = folder.join(format!("poses/{:04}.json", d.id));
            let text =
                std::fs::read_to_string(&file).map_err(|e| format!("{}: {e}", file.display()))?;
            let pose: Pose =
                serde_json::from_str(&text).map_err(|e| format!("{}: {e}", file.display()))?;
            pose.validate()?;
            if pose.id != d.id
                || pose.name != d.name
                || pose.actor != d.actor
                || pose.family != d.family
                || pose.held_contacts != d.held_contacts
                || pose.source_slots != d.source_slots
                || pose.screenshot != d.screenshot
            {
                return Err(format!("descriptor/pose mismatch {}", d.id));
            }
        }
        Ok(catalog)
    }
}
impl Pose {
    pub fn validate(&self) -> Result<(), String> {
        let fail = || format!("invalid skeleton, contacts or matrices in pose {}", self.id);
        let expected = if self.actor == "human" { 20 } else { 10 };
        if self.schema_version != 1
            || self.id != self.frame_id
            || !valid_actor(&self.actor)
            || self.bind.len() != expected
            || self.bone_basis.len() != expected
            || self.bone_world.len() != expected
            || !rigid(&self.root_matrix)
            || !rigid(&self.rig_matrix)
            || !contacts(&self.held_contacts)
            || !contacts(&self.released_contacts)
            || self
                .held_contacts
                .iter()
                .any(|c| self.released_contacts.contains(c))
            || !safe_path(&self.screenshot)
            || self.source_slots.iter().any(|s| *s >= 326)
            || !self.max_attachment_error_m.is_finite()
            || self.max_attachment_error_m > 0.005
            || !self.max_bone_length_error_m.is_finite()
            || self.max_bone_length_error_m > 0.001
        {
            return Err(fail());
        }
        let mut roots = 0;
        for (name, bind) in &self.bind {
            let world = self.bone_world.get(name).ok_or_else(fail)?;
            let basis = self.bone_basis.get(name).ok_or_else(fail)?;
            if !rigid(world)
                || !rigid(basis)
                || !rigid(&bind.matrix_local)
                || distance(bind.head, bind.tail) <= 0.001
                || bind
                    .head
                    .iter()
                    .chain(&bind.tail)
                    .any(|v| !v.is_finite() || v.abs() > 10.0)
            {
                return Err(fail());
            }
            let mut ancestor = bind.parent.as_ref();
            let mut visited = HashSet::from([name]);
            while let Some(parent) = ancestor {
                if !visited.insert(parent) {
                    return Err(fail());
                }
                ancestor = self.bind.get(parent).ok_or_else(fail)?.parent.as_ref();
            }
            if let Some(parent) = &bind.parent {
                let rest = &self.bind.get(parent).ok_or_else(fail)?.matrix_local;
                let expected = point(
                    self.bone_world.get(parent).ok_or_else(fail)?,
                    inverse_point(rest, bind.head),
                );
                let actual = [world[0][3], world[1][3], world[2][3]];
                if distance(expected, actual) > 0.005 {
                    return Err(format!("detached joint {name} in {}", self.id));
                }
            } else {
                roots += 1;
            }
        }
        if roots != 1 {
            return Err(fail());
        }
        for (a, b, limit) in if self.actor == "human" {
            vec![
                ("upper_arm.L", "forearm.L", 2.35),
                ("upper_arm.R", "forearm.R", 2.35),
                ("thigh.L", "shin.L", 2.45),
                ("thigh.R", "shin.R", 2.45),
            ]
        } else {
            vec![]
        } {
            let a = self.bone_world.get(a).ok_or_else(fail)?;
            let b = self.bone_world.get(b).ok_or_else(fail)?;
            let dot: f32 = (0..3).map(|i| a[i][1] * b[i][1]).sum();
            if dot.clamp(-1.0, 1.0).acos() > limit + 0.01 {
                return Err(format!("hinge flex exceeds anatomy in {}", self.id));
            }
        }
        if self
            .contact_errors_m
            .values()
            .any(|v| !v.is_finite() || *v < 0.0 || *v > 0.005)
        {
            return Err(fail());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../art/animation")
    }
    fn catalog() -> Catalog {
        Catalog::from_json(&std::fs::read_to_string(root().join("manifest.json")).unwrap()).unwrap()
    }
    fn pose() -> Pose {
        serde_json::from_str(&std::fs::read_to_string(root().join("poses/0003.json")).unwrap())
            .unwrap()
    }
    #[test]
    fn editable_library_is_connected_finite_and_complete() {
        let c = Catalog::validate_directory(&root().join("manifest.json")).unwrap();
        assert!(c.poses.len() >= 247);
        for family in [
            "combat",
            "locomotion",
            "terrain",
            "ragdoll",
            "recovery",
            "crash",
            "animal",
        ] {
            assert!(c.use_cases.iter().any(|u| u.family == family));
        }
        for p in &c.poses {
            assert!(
                c.use_cases.iter().any(|c| c.pose_ids.contains(&p.id)),
                "unmapped pose {}",
                p.id
            );
        }
    }
    #[test]
    fn invalid_references_duplicate_ids_paths_and_guessed_timing_are_rejected() {
        let mut c = catalog();
        c.poses[1].id = c.poses[0].id;
        assert!(c.validate().is_err());
        c = catalog();
        c.use_cases[0].pose_ids[0] = 65535;
        assert!(c.validate().is_err());
        c = catalog();
        c.poses[0].screenshot = "../secret".into();
        assert!(c.validate().is_err());
        c = catalog();
        c.use_cases[0].timing_basis = "guessed-original".into();
        assert!(c.validate().is_err());
        c = catalog();
        c.use_cases[0].actor = "cow".into();
        assert!(c.validate().is_err());
        c = catalog();
        c.poses[0].source_slots = vec![326];
        assert!(c.validate().is_err());
    }
    #[test]
    fn detached_stretched_nonfinite_and_cyclic_skeletons_are_rejected() {
        let mut p = pose();
        p.bone_world.get_mut("forearm.L").unwrap()[0][3] += 0.2;
        assert!(p.validate().is_err());
        p = pose();
        p.bone_basis.get_mut("shin.L").unwrap()[0][0] = 3.0;
        assert!(p.validate().is_err());
        p = pose();
        p.root_matrix[0][0] = f32::NAN;
        assert!(p.validate().is_err());
        p = pose();
        p.bind.get_mut("root").unwrap().parent = Some("head".into());
        assert!(p.validate().is_err());
        p = pose();
        p.held_contacts.push("engine-impulse".into());
        assert!(p.validate().is_err());
    }
    #[test]
    fn authoring_schema_cannot_carry_physics_or_executable_fields() {
        let mut value = serde_json::to_value(pose()).unwrap();
        value["rigid_body"] = serde_json::json!({"mass":200,"impulse":[0,100,0]});
        assert!(serde_json::from_value::<Pose>(value).is_err());
        let mut c = catalog();
        c.runtime_connected = true;
        assert!(c.validate().is_err());
    }
}
