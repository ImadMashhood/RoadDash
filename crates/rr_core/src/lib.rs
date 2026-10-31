//! Renderer/physics-independent domain contracts shared by the offline client
//! and future authoritative server. No Bevy or Rapier dependency is permitted.
pub mod comfort;
use serde::{Deserialize, Serialize};

pub const SIMULATION_HZ: f64 = 60.0;
pub const SAVE_SCHEMA_VERSION: u32 = 1;
pub const PROTOCOL_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Garage,
    Countdown,
    Racing,
    Paused,
    Finished,
}
impl Phase {
    pub fn simulating(self) -> bool {
        matches!(self, Self::Countdown | Self::Racing)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ContentId(String);
impl ContentId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for ContentId {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty()
            || value.len() > 80
            || !value
                .bytes()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_' || c == b'-')
        {
            return Err(format!("invalid stable content ID: {value:?}"));
        }
        Ok(Self(value))
    }
}
impl From<ContentId> for String {
    fn from(id: ContentId) -> Self {
        id.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VehicleIntent {
    pub throttle: bool,
    pub brake: bool,
    pub steer: f32,
    pub nitro: bool,
    #[serde(default)]
    pub throttle_amount: f32,
    #[serde(default)]
    pub brake_amount: f32,
}
impl VehicleIntent {
    pub fn throttle_strength(self) -> f32 {
        strength(self.throttle, self.throttle_amount)
    }
    pub fn brake_strength(self) -> f32 {
        strength(self.brake, self.brake_amount)
    }
}
fn strength(held: bool, amount: f32) -> f32 {
    if !held {
        0.0
    } else if amount == 0.0 {
        1.0
    } else if amount.is_finite() {
        amount.clamp(0.0, 1.0)
    } else {
        0.0
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AttackSide {
    Left,
    Right,
}
impl AttackSide {
    pub fn sign(self) -> f32 {
        match self {
            Self::Left => -1.0,
            Self::Right => 1.0,
        }
    }
}

/// A render-frame edge survives zero-tick frames and is consumed only once
/// when a fixed tick finally runs. Repeated presses coalesce until consumed.
#[derive(Default, Debug)]
pub struct InputBuffer {
    pub held: VehicleIntent,
    left: bool,
    right: bool,
}
impl InputBuffer {
    pub fn push_attack(&mut self, side: AttackSide) {
        match side {
            AttackSide::Left => self.left = true,
            AttackSide::Right => self.right = true,
        }
    }
    pub fn take_attack(&mut self) -> Option<AttackSide> {
        if self.left {
            self.left = false;
            Some(AttackSide::Left)
        } else if self.right {
            self.right = false;
            Some(AttackSide::Right)
        } else {
            None
        }
    }
    pub fn clear_edges(&mut self) {
        self.left = false;
        self.right = false;
    }
}
pub fn race_award(place: usize, takedowns: u32) -> u32 {
    (9u32.saturating_sub(place as u32))
        .saturating_mul(250)
        .saturating_add(takedowns.saturating_mul(100))
}

/// Hits form a chain when each accepted direct strike is within three seconds
/// of the previous one (0, 2, 4 seconds is a knockout). Duplicate contact in the
/// same attack instant cannot advance the chain twice.
#[derive(Clone, Copy, Debug, Default)]
pub struct HitChain {
    pub count: u8,
    pub last: Option<f32>,
}
impl HitChain {
    pub fn expire(&mut self, now: f32) -> bool {
        if self.last.is_some_and(|last| now - last > 3.0) {
            *self = Self::default();
            true
        } else {
            false
        }
    }
    pub fn direct_hit(&mut self, now: f32) -> bool {
        self.expire(now);
        if self.last.is_some_and(|last| now - last < 0.25) {
            return false;
        }
        self.last = Some(now);
        self.count = (self.count + 1).min(3);
        self.count == 3
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContactSeverity {
    Nudge,
    Wipeout,
}
/// Relative *closing* velocity along the impact normal. Absolute speed alone
/// must not knock out two riders travelling together.
pub fn bike_contact_severity(closing_mps: f32, longitudinal: bool) -> ContactSeverity {
    let threshold = if longitudinal { 50.0 / 3.6 } else { 6.0 };
    if closing_mps >= threshold {
        ContactSeverity::Wipeout
    } else {
        ContactSeverity::Nudge
    }
}
pub fn direct_strike_geometry(lateral: f32, longitudinal: f32, height: f32, side: f32) -> bool {
    (0.65..=1.9).contains(&lateral.abs())
        && lateral * side > 0.0
        && longitudinal.abs() <= 0.9
        && height.abs() <= 0.9
}
#[cfg(test)]
mod combat_rule_tests {
    use super::*;
    #[test]
    fn successive_hits_expire_by_gap_not_total_duration() {
        let mut c = HitChain::default();
        assert!(!c.direct_hit(0.0));
        assert!(!c.direct_hit(2.0));
        assert!(c.direct_hit(4.0));
        let mut c = HitChain::default();
        c.direct_hit(0.0);
        c.direct_hit(3.01);
        assert_eq!(c.count, 1);
        c.direct_hit(3.01);
        assert_eq!(c.count, 1);
    }
    #[test]
    fn impact_classification_uses_relative_normal_speed() {
        assert_eq!(
            bike_contact_severity(49.0 / 3.6, true),
            ContactSeverity::Nudge
        );
        assert_eq!(
            bike_contact_severity(50.0 / 3.6, true),
            ContactSeverity::Wipeout
        );
        assert_eq!(bike_contact_severity(1.5, false), ContactSeverity::Nudge);
        assert_eq!(bike_contact_severity(8.0, false), ContactSeverity::Wipeout);
        assert_eq!(bike_contact_severity(-30.0, true), ContactSeverity::Nudge);
    }
    #[test]
    fn direct_strikes_require_side_by_side_reach() {
        assert!(direct_strike_geometry(1.7, 0.5, 0.1, 1.0));
        assert!(!direct_strike_geometry(1.7, 1.5, 0.1, 1.0));
        assert!(!direct_strike_geometry(2.8, 0.0, 0.0, 1.0));
        assert!(!direct_strike_geometry(1.7, 0.0, 0.0, -1.0));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn edges_survive_render_frames_and_are_consumed_once() {
        let mut b = InputBuffer::default();
        b.push_attack(AttackSide::Left);
        b.push_attack(AttackSide::Right);
        assert_eq!(b.take_attack(), Some(AttackSide::Left));
        assert_eq!(b.take_attack(), Some(AttackSide::Right));
        assert_eq!(b.take_attack(), None);
    }
    #[test]
    fn ids_reject_unstable_or_path_like_values() {
        for bad in ["", "Bike X", "../bike", "UPPER"] {
            assert!(ContentId::try_from(bad.to_string()).is_err());
        }
        assert!(ContentId::try_from("bike-balanced-01".to_string()).is_ok());
    }
    #[test]
    fn rewards_are_bounded_and_match_the_offline_rules() {
        assert_eq!(race_award(1, 2), 2200);
        assert_eq!(race_award(8, 0), 250);
        assert_eq!(race_award(99, 0), 0);
    }
}
