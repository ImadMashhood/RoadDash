//! Transport-independent validation and grid rules for listen-server races.
use crate::{AttackSide, VehicleIntent};
use serde::{Deserialize, Serialize};
pub const RACERS: usize = 15;
pub const PROTOCOL: u32 = 4;
/// Voice chat fades out over 55 meters of world-space separation.
pub fn proximity_voice_gain(distance: f32) -> u8 {
    const RANGE_METERS: f32 = 55.;
    if distance.is_finite() && distance >= 0. {
        ((1. - distance / RANGE_METERS).clamp(0., 1.) * 255.).round() as u8
    } else {
        0
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub sequence: u64,
    pub vehicle: VehicleIntent,
    pub attack: Option<AttackSide>,
    pub foot: [f32; 2],
    pub foot_forward: [f32; 2],
    pub interact: bool,
    pub pickup: bool,
    pub drop: bool,
}
impl Input {
    pub fn valid(&self) -> bool {
        self.vehicle.steer.is_finite()
            && self.vehicle.steer.abs() <= 1.0
            && self.vehicle.throttle_amount.is_finite()
            && (0.0..=1.0).contains(&self.vehicle.throttle_amount)
            && self.vehicle.brake_amount.is_finite()
            && (0.0..=1.0).contains(&self.vehicle.brake_amount)
            && self
                .foot_forward
                .iter()
                .all(|v| v.is_finite() && v.abs() <= 1.0)
            && self.foot.iter().all(|v| v.is_finite() && v.abs() <= 1.0)
    }
    pub fn clear_edges(&mut self) {
        self.attack = None;
        self.interact = false;
        self.pickup = false;
        self.drop = false;
    }
}
pub fn shuffled_grid(mut seed: u64) -> [usize; RACERS] {
    let mut grid = std::array::from_fn(|i| i);
    for i in (1..RACERS).rev() {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        grid.swap(i, seed as usize % (i + 1));
    }
    grid
}
/// Ten RFC4648 base32 digits carry an IPv4 address and a port, without a directory.
pub fn encode_endpoint(ip: [u8; 4], port: u16) -> String {
    let mut bits = ((u32::from_be_bytes(ip) as u64) << 16) | port as u64;
    let mut out = [b'A'; 10];
    for c in out.iter_mut().rev() {
        *c = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"[(bits & 31) as usize];
        bits >>= 5;
    }
    String::from_utf8(out.to_vec()).unwrap()
}
pub fn decode_endpoint(code: &str) -> Option<([u8; 4], u16)> {
    if code.len() != 10 {
        return None;
    }
    let mut bits = 0u64;
    for c in code.bytes() {
        let n = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567"
            .iter()
            .position(|v| *v == c.to_ascii_uppercase())?;
        bits = (bits << 5) | n as u64;
    }
    if bits >> 48 != 0 || bits as u16 == 0 {
        return None;
    }
    Some((((bits >> 16) as u32).to_be_bytes(), bits as u16))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn code_round_trip() {
        for ip in [[127, 0, 0, 1], [192, 168, 20, 55], [255, 255, 255, 255]] {
            for port in [1, 47654, 65535] {
                assert_eq!(
                    decode_endpoint(&encode_endpoint(ip, port)),
                    Some((ip, port))
                );
            }
        }
        assert_eq!(decode_endpoint("ZZZZZZZZZZ"), None);
    }
    #[test]
    fn grid_is_permutation_and_changes() {
        let a = shuffled_grid(37);
        let mut b = a;
        b.sort();
        assert_eq!(b, std::array::from_fn(|i| i));
        assert_ne!(a, shuffled_grid(38));
    }
    #[test]
    fn rejects_non_finite_and_out_of_range_input() {
        let mut input = Input::default();
        assert!(input.valid());
        input.foot[0] = f32::NAN;
        assert!(!input.valid());
        input.foot = [0., 0.];
        input.vehicle.steer = 1.01;
        assert!(!input.valid());
    }
}

/// Quick Play uses measured request/response RTT, inclusive of the user's cap.
pub fn eligible_rtt(elapsed: std::time::Duration) -> bool {
    elapsed <= std::time::Duration::from_millis(250)
}

/// A union of human-view neighborhoods; never used to disable host simulation.
pub fn replication_interest(position: [f32; 3], viewers: &[[f32; 3]], radius: f32) -> bool {
    radius.is_finite()
        && radius > 0.0
        && position.iter().all(|v| v.is_finite())
        && viewers.iter().any(|viewer| {
            viewer.iter().all(|v| v.is_finite())
                && position
                    .iter()
                    .zip(viewer)
                    .map(|(a, b)| (a - b).powi(2))
                    .sum::<f32>()
                    <= radius * radius
        })
}
#[cfg(test)]
mod interest_tests {
    #[test]
    fn includes_each_human_boundary_and_rejects_nonfinite_or_unobserved_space() {
        let viewers = [[0., 0., 0.], [2000., 0., 0.]];
        for (point, expected) in [
            ([450., 0., 0.], true),
            ([451., 0., 0.], false),
            ([1999., 0., 0.], true),
            ([1000., 0., 0.], false),
            ([f32::NAN, 0., 0.], false),
        ] {
            assert_eq!(super::replication_interest(point, &viewers, 450.), expected);
        }
        assert!(!super::replication_interest([0.; 3], &[], 450.));
    }
}
#[cfg(test)]
mod rtt_tests {
    #[test]
    fn inclusive_boundary() {
        for (ms, yes) in [(249, true), (250, true), (251, false)] {
            assert_eq!(
                super::eligible_rtt(std::time::Duration::from_millis(ms)),
                yes
            );
        }
    }
}
