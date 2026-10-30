//! Authoring format; validation requires no engine, renderer, or physics world.
use crate::ContentId;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sector {
    Farms,
    City,
    Hills,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    Asphalt,
    Dirt,
    Gravel,
    Grass,
    Water,
}
impl Surface {
    pub fn traction(self) -> f32 {
        match self {
            Self::Asphalt => 1.0,
            Self::Dirt => 0.75,
            Self::Gravel => 0.65,
            Self::Grass => 0.68,
            Self::Water => 0.2,
        }
    }
    pub fn friction(self) -> f32 {
        match self {
            Self::Asphalt => 0.9,
            Self::Dirt => 0.65,
            Self::Gravel => 0.55,
            Self::Grass => 0.6,
            Self::Water => 0.15,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlPoint {
    pub position: [f32; 3],
    pub half_width: f32,
    pub bank_degrees: f32,
    pub sector: Sector,
    pub surface: Surface,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub id: ContentId,
    pub fraction: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Spawn {
    pub id: ContentId,
    pub distance: f32,
    pub lane: f32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PropKind {
    Farm,
    Fence,
    Cow,
    Building,
    Crossing,
    Pedestrian,
    Guardrail,
    Cliff,
    AnimalCrossing,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prop {
    pub id: ContentId,
    pub kind: PropKind,
    pub fraction: f32,
    pub lane: f32,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrackDef {
    pub schema_version: u32,
    pub id: ContentId,
    pub name: String,
    pub closed: bool,
    #[serde(default)]
    pub scenery_seed: u32,
    pub control_points: Vec<ControlPoint>,
    pub checkpoints: Vec<Checkpoint>,
    pub spawns: Vec<Spawn>,
    pub props: Vec<Prop>,
    pub verge_width: f32,
}
impl TrackDef {
    pub fn from_json(json: &str) -> Result<Self, String> {
        let value: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.control_points.len() < 4
            || self.control_points.len() > 256
        {
            return Err("unsupported track version / control point count".into());
        }
        if !self.verge_width.is_finite() || !(2.0..=20.0).contains(&self.verge_width) {
            return Err("invalid safe verge width".into());
        }
        for (i, p) in self.control_points.iter().enumerate() {
            if !p
                .position
                .iter()
                .all(|x| x.is_finite() && x.abs() < 20000.0)
                || !p.half_width.is_finite()
                || !(3.0..=12.0).contains(&p.half_width)
                || !p.bank_degrees.is_finite()
                || p.bank_degrees.abs() > 15.0
            {
                return Err("invalid coordinate, width or bank".into());
            }
            if !self.closed && i + 1 == self.control_points.len() {
                continue;
            }
            let b = &self.control_points[(i + 1) % self.control_points.len()];
            let span = ((p.position[0] - b.position[0]).powi(2)
                + (p.position[2] - b.position[2]).powi(2))
            .sqrt();
            if span < 15.0 || (p.position[1] - b.position[1]).abs() / span > 0.35 {
                return Err("degenerate segment / excessive grade".into());
            }
        }
        let mut ids = HashSet::new();
        if self.checkpoints.len() < 3 {
            return Err("need start and at least two ordered gates".into());
        }
        let mut previous = -1.0;
        for gate in &self.checkpoints {
            if !ids.insert(gate.id.as_str())
                || !gate.fraction.is_finite()
                || !(0.0..=if self.closed { 0.999999 } else { 1.0 }).contains(&gate.fraction)
                || gate.fraction <= previous
            {
                return Err("invalid checkpoint order or ID".into());
            }
            previous = gate.fraction;
        }
        if !self.closed && self.checkpoints.last().unwrap().fraction != 1.0 {
            return Err("point-to-point course needs a finish gate at 1.0".into());
        }
        if self.checkpoints[0].fraction != 0.0 {
            return Err("first gate must be start/finish".into());
        }
        for spawn in &self.spawns {
            if !ids.insert(spawn.id.as_str())
                || !spawn.distance.is_finite()
                || !spawn.lane.is_finite()
                || spawn.lane.abs() > 4.0
                || spawn.distance > 0.0
                || spawn.distance < -100.0
            {
                return Err("invalid grid spawn".into());
            }
        }
        if self.spawns.len() != 8 {
            return Err("need eight grid spawns".into());
        }
        for (i, a) in self.spawns.iter().enumerate() {
            for b in self.spawns.iter().skip(i + 1) {
                if (a.distance - b.distance).abs() < 3.0 && (a.lane - b.lane).abs() < 1.2 {
                    return Err("spawn overlap".into());
                }
            }
        }
        for prop in &self.props {
            if !ids.insert(prop.id.as_str())
                || !prop.fraction.is_finite()
                || !(0.0..1.0).contains(&prop.fraction)
                || !prop.lane.is_finite()
                || prop.lane.abs() > 200.0
            {
                return Err("invalid prop placement".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> TrackDef {
        TrackDef::from_json(include_str!("../../../content/tracks/handling.json")).unwrap()
    }
    #[test]
    fn authoring_rejects_invalid_geometry_gates_and_spawns() {
        let mut t = fixture();
        t.control_points[0].half_width = -1.0;
        assert!(t.validate().is_err());
        let mut t = fixture();
        t.control_points[0].position[1] = f32::NAN;
        assert!(t.validate().is_err());
        let mut t = fixture();
        t.checkpoints.swap(1, 2);
        assert!(t.validate().is_err());
        let mut t = fixture();
        t.spawns[1] = t.spawns[0].clone();
        assert!(t.validate().is_err());
        let mut t = fixture();
        t.control_points[1].position = t.control_points[0].position;
        assert!(t.validate().is_err());
    }
    #[test]
    fn all_surface_metadata_is_finite_and_grass_is_driveable() {
        for s in [
            Surface::Asphalt,
            Surface::Dirt,
            Surface::Gravel,
            Surface::Grass,
            Surface::Water,
        ] {
            assert!(s.traction() > 0.0 && s.friction() > 0.0);
        }
        assert!(Surface::Grass.traction() > 0.5);
    }
}
