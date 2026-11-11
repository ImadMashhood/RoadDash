//! Shared surface rules, independent of rendering and physics adapters.
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Surface {
    Asphalt,
    Dirt,
    Gravel,
    Grass,
    Water,
    Sand,
}
impl Surface {
    pub fn traction(self) -> f32 {
        match self {
            Self::Asphalt => 1.0,
            Self::Dirt => 0.75,
            Self::Gravel => 0.65,
            Self::Grass => 0.68,
            Self::Water => 0.2,
            Self::Sand => 0.50,
        }
    }
    pub fn friction(self) -> f32 {
        match self {
            Self::Asphalt => 0.9,
            Self::Dirt => 0.65,
            Self::Gravel => 0.55,
            Self::Grass => 0.6,
            Self::Water => 0.15,
            Self::Sand => 0.50,
        }
    }
}

impl Surface {
    /// Existing non-sand limits and traction remain unchanged. Current courses do
    /// not author Water as a riding surface; its legacy metadata stays intact.
    pub fn speed_limit(self, road_limit: f32) -> f32 {
        match self {
            Self::Grass => road_limit.min(32.0),
            Self::Sand => road_limit.min(18.0),
            _ => road_limit,
        }
    }
    pub fn drive_scale(self) -> f32 {
        if self == Self::Sand { 0.55 } else { 1.0 }
    }
    /// Budget resistance after the already requested tyre/aero acceleration.
    /// Two independently stop-bounded forces must not add up to reverse motion.
    pub fn rolling_after_drive(self, speed: f32, acceleration: f32, dt: f32) -> f32 {
        let predicted = speed + acceleration * dt;
        if predicted * speed > 0.0 {
            self.rolling_acceleration(predicted, dt)
        } else {
            0.0
        }
    }
    /// Dissipative rolling resistance in m/s². Never reverses velocity at rest.
    /// Separate from tyre grip: soft ground also slows a freely coasting wheel.
    pub fn rolling_acceleration(self, speed: f32, dt: f32) -> f32 {
        if self != Self::Sand || dt <= 0.0 {
            return 0.0;
        }
        -speed.signum() * (1.8 + 0.14 * speed.abs()).min(speed.abs() / dt)
    }
}

/// Hills' authored meadow-to-sand transition, shared by geometry and simulation.
pub fn hills_meadow_weight(station: f32) -> f32 {
    ((5500.0 - station) / 300.0).clamp(0.0, 1.0)
}
pub fn hills_ground(station: f32) -> Surface {
    if hills_meadow_weight(station) <= 0.5 {
        Surface::Sand
    } else {
        Surface::Grass
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sand_is_strongest_driveable_penalty_and_never_adds_energy() {
        for other in [
            Surface::Asphalt,
            Surface::Dirt,
            Surface::Gravel,
            Surface::Grass,
        ] {
            assert!(Surface::Sand.speed_limit(55.) < other.speed_limit(55.));
            assert!(
                Surface::Sand.drive_scale() * Surface::Sand.traction()
                    < other.drive_scale() * other.traction()
            );
        }
        for speed in [-50., -3., -0.001, 0., 0.001, 3., 50.] {
            let next = speed + Surface::Sand.rolling_acceleration(speed, 1. / 60.) / 60.;
            assert!(next.abs() <= speed.abs());
            assert!(next * speed >= 0.);
        }
    }
    #[test]
    fn hills_surface_uses_the_same_authored_blend_as_presentation() {
        for station in [5200., 5349., 5350., 5500., 8500., 12000.] {
            assert_eq!(
                hills_ground(station) == Surface::Sand,
                hills_meadow_weight(station) <= 0.5
            );
        }
    }
}

#[cfg(test)]
mod combined_stop {
    use super::*;
    #[test]
    fn combined_engine_and_sand_drag_do_not_reverse_a_coasting_bike() {
        for initial in [-3., -0.03, -0.001, 0., 0.001, 0.03, 3.] {
            let mut speed: f32 = initial;
            let dt = 1. / 60.;
            for _ in 0..180 {
                let engine = -speed.signum() * 1.8_f32.min(speed.abs() / dt);
                speed += (engine + Surface::Sand.rolling_after_drive(speed, engine, dt)) * dt;
                assert!(initial * speed >= -1e-7);
            }
            assert!(speed.abs() < 0.001);
        }
    }
}
