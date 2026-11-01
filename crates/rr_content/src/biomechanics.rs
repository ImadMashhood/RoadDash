//! Centralized, bounded ragdoll authoring parameters in SI units/radians.
use serde::Deserialize;
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Joint {
    pub name: String,
    pub limits: [[f32; 2]; 3],
    pub rest: [f32; 3],
    pub brace: [f32; 3],
    pub passive_k: f32,
    pub passive_d: f32,
    pub active_k: f32,
    pub active_d: f32,
    pub passive_torque: f32,
    pub active_torque: f32,
    pub hinge: bool,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub activation: rr_core::biomechanics::Activation,
    pub masses: [f32; 16],
    pub joints: Vec<Joint>,
    pub extra_solver_iterations: usize,
    pub physics_substeps: usize,
    pub linear_damping: f32,
    pub angular_damping: f32,
}
impl Config {
    pub fn from_json(text: &str) -> Result<Self, String> {
        let c: Self = serde_json::from_str(text).map_err(|e| e.to_string())?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<(), String> {
        let a = &self.activation;
        if self.version != 1
            || self.joints.len() != 16
            || self.extra_solver_iterations > 16
            || !(1..=8).contains(&self.physics_substeps)
            || self
                .masses
                .iter()
                .any(|m| !m.is_finite() || !(0.25..=30.0).contains(m))
            || (self.masses.iter().sum::<f32>() - 70.0).abs() > 0.01
            || [self.linear_damping, self.angular_damping]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=3.0).contains(v))
            || [a.riding, a.bracing, a.ejection, a.tumbling]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
            || [
                a.impact_seconds,
                a.ejection_seconds,
                a.tumbling_fade_seconds,
                a.settle_seconds,
                a.resting_linear,
                a.resting_angular,
            ]
            .iter()
            .any(|v| !v.is_finite() || !(0.01..=5.0).contains(v))
        {
            return Err("invalid biomechanical configuration".into());
        }
        for j in &self.joints {
            if j.name.is_empty()
                || [
                    j.passive_k,
                    j.passive_d,
                    j.active_k,
                    j.active_d,
                    j.passive_torque,
                    j.active_torque,
                ]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=300.0).contains(v))
                || j.passive_k <= 0.0
                || j.passive_d <= 0.0
                || j.passive_torque <= 0.0
            {
                return Err("invalid muscle gains".into());
            }
            for i in 0..3 {
                let [lo, hi] = j.limits[i];
                if !lo.is_finite()
                    || !hi.is_finite()
                    || lo > hi
                    || lo < -3.0
                    || hi > 3.0
                    || [j.rest[i], j.brace[i]]
                        .iter()
                        .any(|v| !v.is_finite() || *v < lo || *v > hi)
                {
                    return Err(format!("invalid joint limits: {}", j.name));
                }
            }
            let chart_radius_squared = j
                .limits
                .iter()
                .map(|[lo, hi]| (lo.abs().max(hi.abs()) * 0.5).sin().powi(2))
                .sum::<f32>();
            if !j.hinge && chart_radius_squared >= 0.95 {
                return Err("ball joint chart approaches quaternion hemisphere singularity".into());
            }
            if j.hinge && (j.limits[1] != [0.0; 2] || j.limits[2] != [0.0; 2]) {
                return Err("hinge may only flex about local X".into());
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shipped_configuration_and_rejection() {
        let text = include_str!("../../../content/biomechanics.json");
        let c = Config::from_json(text).unwrap();
        assert!((c.masses.iter().sum::<f32>() - 70.0).abs() < 0.01);
        for mutation in 0..5 {
            let mut d = c.clone();
            match mutation {
                0 => d.masses[0] = 0.0,
                1 => d.activation.impact_seconds = 0.0,
                2 => d.joints[4].limits[1] = [-1.0, 1.0],
                3 => d.joints[1].passive_d = f32::NAN,
                _ => d.joints[3].limits = [[-2.5, 2.5]; 3],
            };
            assert!(d.validate().is_err());
        }
    }
}
