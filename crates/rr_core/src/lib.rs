//! Renderer/physics-independent domain contracts shared by the offline client
//! and future authoritative server. No Bevy or Rapier dependency is permitted.
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
