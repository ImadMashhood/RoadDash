//! Presentation timing and original riding-pose selection; no physics or renderer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RidePose {
    LowSpeed,
    Acceleration,
    Normal,
    Tuck,
}
/// Original selector (actor_animation.cpp), using original fixed-point units.
pub fn reference_ride_pose(
    speed: i32,
    throttle: i32,
    lateral: i32,
    flags: u8,
    boost: i32,
) -> RidePose {
    if speed < 300 {
        RidePose::LowSpeed
    } else if speed < 0x708 && throttle > 30 {
        RidePose::Acceleration
    } else if (flags & 0x20 == 0 || boost > 99) && (lateral < -0x1000 || speed >= 0x251d) {
        RidePose::Tuck
    } else {
        RidePose::Normal
    }
}
/// Inverse of the original HUD's unrounded mph coefficients: 41 / 4096.
/// Rendering policy only; this conversion never changes motorcycle speed.
pub fn reference_velocity(mps: f32) -> i32 {
    (mps.max(0.0) / 0.44704 * 4096.0 / 41.0).round() as i32
}
pub fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
pub fn approach(current: f32, target: f32, dt: f32, seconds: f32) -> f32 {
    current + (target - current) * (1.0 - (-dt.max(0.0) / seconds).exp())
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhaseSample {
    pub from: usize,
    pub to: usize,
    pub blend: f32,
}
/// Sample time between source phases; a repeated image is still a timed phase.
pub fn sample_phase(durations: &[f32], seconds: f32) -> PhaseSample {
    assert!(!durations.is_empty());
    let mut t = seconds.max(0.0);
    for (i, &d) in durations.iter().enumerate() {
        if t < d || i + 1 == durations.len() {
            return PhaseSample {
                from: i,
                to: (i + 1).min(durations.len() - 1),
                blend: smoothstep(t / d),
            };
        }
        t -= d;
    }
    unreachable!()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_selector_boundaries_and_boost_gate() {
        assert_eq!(reference_ride_pose(299, 100, 0, 0, 0), RidePose::LowSpeed);
        assert_eq!(
            reference_ride_pose(300, 31, 0, 0, 0),
            RidePose::Acceleration
        );
        assert_eq!(reference_ride_pose(1799, 30, 0, 0, 0), RidePose::Normal);
        assert_eq!(reference_ride_pose(9500, 100, 0, 0, 0), RidePose::Normal);
        assert_eq!(reference_ride_pose(9501, 100, 0, 0, 0), RidePose::Tuck);
        assert_eq!(
            reference_ride_pose(9501, 100, 0, 0x20, 99),
            RidePose::Normal
        );
        assert_eq!(reference_ride_pose(9501, 100, 0, 0x20, 100), RidePose::Tuck);
        assert_eq!(reference_ride_pose(2000, 0, -4097, 0, 0), RidePose::Tuck);
    }
    #[test]
    fn phase_order_repeats_and_end_are_preserved() {
        let ds = [0.14, 0.12, 0.14];
        assert_eq!(sample_phase(&ds, 0.0).from, 0);
        assert_eq!(sample_phase(&ds, 0.14).from, 1);
        assert_eq!(sample_phase(&ds, 99.0).from, 2);
        assert_eq!(sample_phase(&ds, 99.0).to, 2);
    }
    #[test]
    fn smoothing_is_frame_rate_independent() {
        let mut results = vec![];
        for hz in [30, 60, 120] {
            let mut x = 0.0;
            for _ in 0..hz {
                x = approach(x, 1.0, 1.0 / hz as f32, 0.18);
            }
            results.push(x);
        }
        assert!((results[0] - results[2]).abs() < 1e-6);
        assert!((reference_velocity(42.51) - 9501).abs() < 5);
    }
}

/// Shared visual/gameplay attack clock. Contact occurs at an explicitly identified authored key.
/// Advancing this clock is an authoritative fixed-tick operation; rendering only samples it.
pub const ATTACK_ENTER: f32 = 0.055;
pub const ATTACK_PLAYBACK: f32 = 1.8;
#[derive(Clone, Copy, Debug)]
pub struct StrikeClock {
    pub age: f32,
    pub contact: f32,
    pub side: f32,
    pub action: crate::AttackSide,
    pub weapon: Option<crate::weapons::Ability>,
    fired: bool,
}
impl StrikeClock {
    pub fn new(action: crate::AttackSide, side: f32, source_contact: f32) -> Self {
        Self {
            age: 0.0,
            contact: ATTACK_ENTER + source_contact / ATTACK_PLAYBACK,
            side,
            action,
            weapon: None,
            fired: false,
        }
    }
    pub fn pending(self) -> bool {
        !self.fired
    }
    pub fn contact_window(self) -> (f32, f32) {
        (self.contact - 0.055, self.contact + 0.10)
    }

    pub fn step(&mut self, dt: f32) {
        self.age += dt.max(0.);
    }

    pub fn consume(&mut self) {
        self.fired = true;
    }

    /// Returns true exactly once, including when a coarse tick crosses contact.
    pub fn advance(&mut self, dt: f32) -> bool {
        self.age += dt.max(0.0);
        if !self.fired && self.age >= self.contact {
            self.fired = true;
            true
        } else {
            false
        }
    }
}

/// One confirmed mounted impact, sampled on the race clock. Presentation only.
/// Replacement rather than accumulation bounds repeated/opposite-side hits.
#[derive(Clone, Copy, Debug)]
pub struct HitReaction {
    pub at: f32,
    pub direction: f32,
    pub kicking: bool,
}
impl HitReaction {
    pub fn new(at: f32, local_away_x: f32, kicking: bool) -> Self {
        Self {
            at,
            direction: if local_away_x < 0.0 { -1.0 } else { 1.0 },
            kicking,
        }
    }
    pub fn active(self, now: f32) -> bool {
        let age = now - self.at;
        age.is_finite() && (0.0..self.duration()).contains(&age)
    }
    pub fn duration(self) -> f32 {
        if self.kicking { 0.38 } else { 0.30 }
    }
    /// Immediate small contact response, peak at 50ms, smooth return to neutral.
    pub fn weight(self, now: f32) -> f32 {
        if !self.active(now) {
            return 0.0;
        }
        let age = now - self.at;
        if age < 0.05 {
            0.25 + 0.75 * smoothstep(age / 0.05)
        } else {
            1.0 - smoothstep((age - 0.05) / (self.duration() - 0.05))
        }
    }
}
#[cfg(test)]
mod hit_reaction_tests {
    use super::*;
    #[test]
    fn impact_pulse_is_bounded_mirrored_finite_and_expires() {
        for kicking in [false, true] {
            for side in [-1.0, 1.0] {
                let hit = HitReaction::new(4.0, side, kicking);
                assert_eq!(hit.direction, side);
                assert_eq!(hit.weight(4.0), 0.25);
                assert!((hit.weight(4.05) - 1.0).abs() < 1e-5);
                for step in 0..1000 {
                    let w = hit.weight(3.9 + step as f32 / 1000.0);
                    assert!((0.0..=1.0).contains(&w));
                }
                for now in [0.0, 5.0, f32::NAN, f32::INFINITY] {
                    assert_eq!(hit.weight(now), 0.0);
                }
            }
        }
    }
}
