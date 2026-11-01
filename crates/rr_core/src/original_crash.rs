//! Independent behavioral model of the original game's integer crash equations.
//! See docs/research/ORIGINAL_CRASH_CALCULATIONS.md for the pinned evidence.
//! Values are native units, NOT SI; geometry, sound/RNG and network ownership
//! remain caller responsibilities. This module contains no reference source.
use serde::{Deserialize, Serialize};

/// HUD-derived longitudinal conversion. Do not use for lateral/vertical channels.
pub const FORWARD_MPS_PER_UNIT: f32 = 0.44704 * 41.0 / 4096.0;
/// Modern adapter calibration: native gravity 74/tick at 60 Hz equals 9.81 m/s².
/// This is a RoadDash choice, not a recovered original metre scale.
pub const VERTICAL_MPS_PER_UNIT: f32 = 9.81 / (74.0 * 60.0);
pub const CONTACT_GAP: i32 = 6001;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Motion {
    pub forward: i32,
    pub lateral: i32,
    pub vertical: i32,
}
impl Motion {
    fn add(self, rhs: Self) -> Self {
        Self {
            forward: self.forward.wrapping_add(rhs.forward),
            lateral: self.lateral.wrapping_add(rhs.lateral),
            vertical: self.vertical.wrapping_add(rhs.vertical),
        }
    }
    pub fn opposite(self) -> Self {
        Self {
            forward: self.forward.wrapping_neg(),
            lateral: self.lateral.wrapping_neg(),
            vertical: self.vertical.wrapping_neg(),
        }
    }
}

/// One selected separation axis. The partner receives the NEGATIVE of this
/// result, not a second independently mass-weighted impulse. Reject bad masses.
/// Static partners divide their mass ratio BEFORE multiplication (quantized).
pub fn separation_delta(v1: i32, v2: i32, m1: i32, m2: i32, static_partner: bool) -> Option<i32> {
    let total = m1.checked_add(m2)?;
    if m1 <= 0 || m2 <= 0 || total <= 0 {
        return None;
    }
    let relative = v2.wrapping_sub(v1);
    Some(if static_partner {
        relative.wrapping_mul(m2.wrapping_mul(2) / total)
    } else {
        relative.wrapping_mul(m2).wrapping_mul(2) / total
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Entry {
    Damage,
    Punch,
    SpeedGap,
    Wall,
    Traction,
}
impl Entry {
    /// (rider, motorcycle). Applied after copying the incoming bike motion.
    pub fn impulses(self) -> (Motion, Motion) {
        let (r, b) = match self {
            Self::Damage => ([-256, 0, 1792], [0, 0, 1024]),
            Self::Punch => ([-80, 0, 0], [0, 0, 0]),
            Self::SpeedGap => ([0, 0, 512], [0, 0, 256]),
            Self::Wall => ([0, 0, 80], [0, 0, 16]),
            Self::Traction => ([0, 0, 0], [0, 0, 0]),
        };
        let motion = |v: [i32; 3]| Motion {
            forward: v[0],
            lateral: v[1],
            vertical: v[2],
        };
        (motion(r), motion(b))
    }
    pub fn apply(self, incoming: Motion) -> (Motion, Motion) {
        let (r, b) = self.impulses();
        (incoming.add(r), incoming.add(b))
    }
}

/// Explicit inputs: do not mistake illustrative defaults for an original rider's
/// BIKESPEC or personality table. RoadDash's adapter supplies its own condition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Condition {
    pub base_stamina: i32,
    pub max_stamina: i32,
    pub stamina: i32,
    pub bike: i32,
}
impl Condition {
    pub fn damage(&mut self, stamina: i32, bike: i32, reason: u8) -> bool {
        let scale = |v: i32| match reason {
            1 | 4 | 5 => v,
            2 => v.wrapping_mul(7) / 4,
            3 => v / 8,
            _ => 0,
        };
        let p = scale(stamina);
        self.max_stamina = self
            .max_stamina
            .wrapping_sub(p.wrapping_mul(4))
            .max(self.base_stamina / 2);
        self.stamina = self.stamina.wrapping_sub(p.wrapping_mul(64)).max(0);
        self.bike = self.bike.wrapping_sub(scale(bike)).max(0);
        self.stamina == 0 || self.bike == 0
    }
}

/// Classes 1/2/3 in the contact handler. Low obstacles use a separate ride-over
/// path and are deliberately not mislabeled as tall contacts here.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactClass {
    Bike = 1,
    Tall = 2,
    Glance = 3,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Contact {
    pub incoming: Motion,
    pub raw_delta: Motion,
    pub partner_gap_speed: i32,
    pub elasticity_q8: i32,
    pub class: ContactClass,
    pub car: bool,
    pub partner_on_left: bool,
    pub condition: Condition,
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Response {
    pub bike: Motion,
    pub rider: Option<Motion>,
    pub condition: Condition,
    pub raw_magnitude: i32,
    pub scaled_delta: Motion,
    pub entry: Option<Entry>,
}
/// Offline, initially riding contact path. Original ordering matters: damage
/// crash copies the rider BEFORE the tall-contact transfer changes bike speed.
pub fn contact(c: Contact) -> Response {
    let full = c
        .raw_delta
        .forward
        .wrapping_abs()
        .wrapping_add(c.raw_delta.lateral.wrapping_abs())
        .wrapping_add(c.raw_delta.vertical.wrapping_abs());
    let magnitude = full >> 10;
    let elastic = |v: i32| ((i64::from(v) * i64::from(c.elasticity_q8)) / 256) as i32;
    let scaled = Motion {
        forward: elastic(c.raw_delta.forward),
        lateral: elastic(c.raw_delta.lateral),
        vertical: elastic(c.raw_delta.vertical),
    };
    let mut condition = c.condition;
    let down = condition.damage(
        if c.car { magnitude } else { 0 },
        if c.car { magnitude } else { magnitude >> 4 },
        c.class as u8,
    );
    let mut bike = c.incoming;
    let mut rider = None;
    let mut entry = None;
    if down {
        let (r, b) = Entry::Damage.apply(bike);
        rider = Some(r);
        bike = b;
        entry = Some(Entry::Damage);
    }
    if c.class == ContactClass::Glance {
        let shove = scaled.forward.wrapping_abs();
        if down {
            bike.vertical = bike.vertical.wrapping_add(shove / 256);
        } else {
            bike.lateral = if c.partner_on_left {
                bike.lateral.max(0).wrapping_add(shove / 2)
            } else {
                bike.lateral.min(0).wrapping_sub(shove / 2)
            };
            bike.forward = bike.forward.wrapping_sub(shove / 4).max(0);
        }
    } else if down {
        if c.class == ContactClass::Tall {
            bike.forward = bike.forward.wrapping_add(scaled.forward);
        }
    } else if bike.forward.wrapping_sub(c.partner_gap_speed) < CONTACT_GAP {
        bike.forward = bike.forward.wrapping_add(scaled.forward);
    } else {
        let (r, b) = Entry::SpeedGap.apply(bike);
        rider = Some(r);
        bike = b;
        entry = Some(Entry::SpeedGap);
    }
    bike.forward = bike.forward.clamp(-1000, 30000);
    bike.vertical = bike.vertical.clamp(-10000, 1536);
    Response {
        bike,
        rider,
        condition,
        raw_magnitude: magnitude,
        scaled_delta: scaled,
        entry,
    }
}

/// Original scalar drag step for a detached actor. Native dt=1 at 60 Hz.
/// Input surface_drag is a native table coefficient, not a friction coefficient.
#[allow(clippy::too_many_arguments)]
pub fn drag_step(
    speed: i32,
    dt: i32,
    grounded: bool,
    surface_drag: i32,
    drag_gain: i32,
    sliding: i32,
    lateral_force: i32,
    throttle: i32,
) -> (i32, i32) {
    let absolute = speed.wrapping_abs();
    let base = if grounded { absolute } else { absolute >> 2 };
    let contact = if grounded {
        (surface_drag.wrapping_mul(absolute) >> 14).wrapping_add(sliding)
    } else {
        0
    };
    let drag = (drag_gain.wrapping_mul(base >> 8) >> 8).wrapping_add(contact);
    let lateral = lateral_force.wrapping_abs() >> 4;
    let acceleration = if speed >= 0 {
        throttle.wrapping_sub(drag).wrapping_sub(lateral)
    } else {
        throttle.wrapping_add(lateral.wrapping_add(drag).wrapping_abs())
    };
    let next = speed.wrapping_add(dt.wrapping_mul(acceleration));
    let result = if speed < 0 {
        next.min(0)
    } else if next < 1 {
        if throttle < 1 { 0 } else { 100 }
    } else {
        next
    };
    (result, acceleration)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> Contact {
        Contact {
            incoming: Motion {
                forward: 12000,
                lateral: 0,
                vertical: 0,
            },
            raw_delta: Motion {
                forward: -18000,
                lateral: 0,
                vertical: 0,
            },
            partner_gap_speed: 4000,
            elasticity_q8: 192,
            class: ContactClass::Tall,
            car: true,
            partner_on_left: true,
            condition: Condition {
                base_stamina: 2500,
                max_stamina: 2500,
                stamina: 2500,
                bike: 1000,
            },
        }
    }
    #[test]
    fn moving_and_static_rounding_are_distinct_and_partner_is_opposite() {
        assert_eq!(separation_delta(9000, 0, 100, 1800, false), Some(-17052));
        assert_eq!(separation_delta(9000, 0, 100, 1800, true), Some(-9000));
        assert_eq!(separation_delta(0, 1, 100, 100, false), Some(1));
        assert_eq!(separation_delta(0, 1, 0, 100, false), None);
        assert_eq!(separation_delta(0, 1, i32::MAX, 100, false), None);
        assert_eq!(
            Motion {
                forward: -17052,
                ..Motion::default()
            }
            .opposite()
            .forward,
            17052
        );
    }
    #[test]
    fn damage_is_unscaled_and_can_copy_rider_before_bike_reverses() {
        let mut c = fixture();
        c.condition.stamina = 1;
        let r = contact(c);
        assert_eq!(r.raw_magnitude, 17);
        assert_eq!(r.condition.stamina, 0);
        assert_eq!(r.condition.max_stamina, 2384); // floor(17*7/4)=29
        assert_eq!(
            r.rider.unwrap(),
            Motion {
                forward: 11744,
                lateral: 0,
                vertical: 1792
            }
        );
        assert_eq!(r.bike.forward, -1000); // 12000 - 13500, then tail clamp
        assert_eq!(r.bike.vertical, 1024);
    }
    #[test]
    fn speed_gap_boundary_and_intact_vs_damage_entry() {
        let mut c = fixture();
        c.raw_delta = Motion::default();
        c.partner_gap_speed = 6000;
        assert!(contact(c).entry.is_none());
        c.partner_gap_speed = 5999;
        let r = contact(c);
        assert_eq!(r.entry, Some(Entry::SpeedGap));
        assert_eq!(r.rider.unwrap().vertical, 512);
        assert_eq!(r.bike.vertical, 256);
    }
    #[test]
    fn glance_bleeds_speed_and_pushes_away_without_automatic_knockoff() {
        let mut c = fixture();
        c.class = ContactClass::Glance;
        let r = contact(c);
        assert!(r.rider.is_none());
        assert_eq!(r.bike.forward, 8625);
        assert_eq!(r.bike.lateral, 6750);
        assert_eq!(r.condition.stamina, 2372);
        c.partner_on_left = false;
        assert_eq!(contact(c).bike.lateral, -6750);
    }
    #[test]
    fn bike_contact_damages_condition_not_stamina_and_class_one_keeps_speed() {
        let mut c = fixture();
        c.car = false;
        c.class = ContactClass::Bike;
        c.condition.bike = 1;
        let r = contact(c);
        assert_eq!(r.condition.stamina, 2500);
        assert_eq!(r.bike.forward, 12000);
        assert_eq!(r.rider.unwrap().forward, 11744);
    }
    #[test]
    fn reason_scaling_and_stamina_floor() {
        for (reason, expected) in [(0, 0), (1, 8), (2, 14), (3, 1), (4, 8), (5, 8), (6, 0)] {
            let mut state = fixture().condition;
            state.damage(8, 8, reason);
            assert_eq!(state.stamina, 2500 - expected * 64);
            assert_eq!(state.bike, 1000 - expected);
        }
        let mut state = fixture().condition;
        assert!(state.damage(10000, 10000, 2));
        assert_eq!(state.max_stamina, 1250);
    }
    #[test]
    fn all_entry_paths_and_contact_clamps_are_explicit() {
        assert_eq!(Entry::Punch.apply(Motion::default()).0.forward, -80);
        assert_eq!(Entry::Wall.apply(Motion::default()).0.vertical, 80);
        assert_eq!(
            Entry::Traction.apply(Motion::default()).0,
            Motion::default()
        );
        let mut c = fixture();
        c.incoming.vertical = 3000;
        c.condition.stamina = 1;
        let r = contact(c);
        assert_eq!(r.bike.vertical, 1536);
        assert_eq!(r.rider.unwrap().vertical, 4792); // rider not subject to bike tail clamp
    }
    #[test]
    fn drag_dissipates_stops_and_uses_quarter_base_while_airborne() {
        assert_eq!(drag_step(12000, 1, false, 0, 130, 70, 0, 0), (11995, -5));
        assert_eq!(drag_step(12000, 1, true, 0, 130, 70, 0, 0), (11907, -93));
        assert_eq!(drag_step(10, 1, true, 0, 130, 70, 0, 0).0, 0);
        assert_eq!(drag_step(-10, 1, true, 0, 130, 70, 0, 0).0, 0);
        let mut v = 12000;
        for _ in 0..1000 {
            v = drag_step(v, 1, true, 64, 130, 70, 0, 0).0;
        }
        assert_eq!(v, 0);
    }
    #[test]
    fn original_is_not_absolute_speed_subtraction() {
        let f = FORWARD_MPS_PER_UNIT;
        let rider = (120.0 / 3.6 / f).round() as i32;
        let car = (40.0 / 3.6 / f).round() as i32;
        let rear = separation_delta(rider, car, 100, 1800, false).unwrap();
        let head = separation_delta(rider, -car, 100, 1800, false).unwrap();
        assert!(head < rear);
        assert!(head.abs() > rider);
    }
}
