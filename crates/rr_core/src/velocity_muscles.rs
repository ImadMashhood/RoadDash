//! Method B: finite-work reciprocal angular impulses. No linear motion is authored.
use serde::{Deserialize, Serialize};
pub type Vector = [f64; 3];
pub type Tensor = [[f64; 3]; 3];
fn dot(a: Vector, b: Vector) -> f64 {
    a.into_iter().zip(b).map(|(a, b)| a * b).sum()
}
fn mul(a: Tensor, v: Vector) -> Vector {
    a.map(|row| dot(row, v))
}
fn scale(v: Vector, s: f64) -> Vector {
    v.map(|x| x * s)
}
fn positive_tensor(m: Tensor) -> bool {
    let tolerance = m.into_iter().flatten().map(f64::abs).fold(1.0, f64::max) * 1e-5;
    (m[0][1] - m[1][0]).abs() <= tolerance
        && (m[0][2] - m[2][0]).abs() <= tolerance
        && (m[1][2] - m[2][1]).abs() <= tolerance
        && m[0][0] > 0.0
        && m[0][0] * m[1][1] - m[0][1] * m[1][0] > 0.0
        && inverse(m).is_some()
}
fn inverse(m: Tensor) -> Option<Tensor> {
    let [[a, b, c], [d, e, f], [g, h, i]] = m;
    let determinant = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
    if !determinant.is_finite() || determinant <= 1e-12 {
        return None;
    }
    Some(
        [
            [e * i - f * h, c * h - b * i, b * f - c * e],
            [f * g - d * i, a * i - c * g, c * d - a * f],
            [d * h - e * g, b * g - a * h, a * e - b * d],
        ]
        .map(|r| r.map(|x| x / determinant)),
    )
}
#[derive(Clone, Copy, Debug)]
pub struct Impulse {
    pub impulse: Vector,
    pub child_delta: Vector,
    pub parent_delta: Vector,
    pub positive_work: f64,
}
/// Solve a bounded relative angular-velocity change. World inverse inertia
/// tensors ensure equal-and-opposite impulses preserve pair angular momentum.
#[allow(clippy::too_many_arguments)]
pub fn drive(
    child_omega: Vector,
    parent_omega: Vector,
    child_inverse: Tensor,
    parent_inverse: Tensor,
    wanted_relative: Vector,
    max_impulse: f64,
    work_available: f64,
) -> Option<Impulse> {
    if child_omega
        .into_iter()
        .chain(parent_omega)
        .chain(wanted_relative)
        .chain(child_inverse.into_iter().flatten())
        .chain(parent_inverse.into_iter().flatten())
        .chain([max_impulse, work_available])
        .any(|x| !x.is_finite())
        || max_impulse < 0.0
        || work_available < 0.0
        || !positive_tensor(child_inverse)
        || !positive_tensor(parent_inverse)
    {
        return None;
    }
    let combined = std::array::from_fn(|i| {
        std::array::from_fn(|j| child_inverse[i][j] + parent_inverse[i][j])
    });
    let relative = std::array::from_fn(|i| child_omega[i] - parent_omega[i]);
    let correction = std::array::from_fn(|i| wanted_relative[i] - relative[i]);
    let mut impulse = mul(inverse(combined)?, correction);
    let length = dot(impulse, impulse).sqrt();
    if length > max_impulse && length > 0.0 {
        impulse = scale(impulse, max_impulse / length);
    }
    let a = 0.5 * dot(impulse, mul(combined, impulse));
    let b = dot(impulse, relative);
    let mut factor = 1.0;
    if a + b > work_available {
        factor = if a > 1e-16 {
            ((-b + (b * b + 4.0 * a * work_available).sqrt()) / (2.0 * a)).clamp(0.0, 1.0)
        } else if b > 0.0 {
            (work_available / b).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }
    impulse = scale(impulse, factor);
    let work = (a * factor * factor + b * factor).max(0.0);
    Some(Impulse {
        impulse,
        child_delta: mul(child_inverse, impulse),
        parent_delta: scale(mul(parent_inverse, impulse), -1.0),
        positive_work: work,
    })
}
/// A supported body guide reacts against the fixed ground. This is not a
/// closed-ragdoll angular-momentum-preserving impulse; its work is bounded and
/// the caller must prove genuine support and exclude obstructions/airborne use.
pub fn support_drive(
    omega: Vector,
    inverse_inertia: Tensor,
    desired_omega: Vector,
    max_impulse: f64,
    work_available: f64,
) -> Option<Impulse> {
    if !positive_tensor(inverse_inertia)
        || omega
            .into_iter()
            .chain(desired_omega)
            .chain([max_impulse, work_available])
            .any(|v| !v.is_finite())
        || max_impulse < 0.0
        || work_available < 0.0
    {
        return None;
    }
    let delta = std::array::from_fn(|i| desired_omega[i] - omega[i]);
    let mut j = mul(inverse(inverse_inertia)?, delta);
    let length = dot(j, j).sqrt();
    if length > max_impulse && length > 0.0 {
        j = scale(j, max_impulse / length);
    }
    let a = 0.5 * dot(j, mul(inverse_inertia, j));
    let b = dot(j, omega);
    let factor = if a + b <= work_available {
        1.0
    } else if a > 1e-16 {
        ((-b + (b * b + 4.0 * a * work_available).sqrt()) / (2.0 * a)).clamp(0.0, 1.0)
    } else if b > 0.0 {
        (work_available / b).clamp(0.0, 1.0)
    } else {
        0.0
    };
    j = scale(j, factor);
    Some(Impulse {
        impulse: j,
        child_delta: mul(inverse_inertia, j),
        parent_delta: [0.0; 3],
        positive_work: (a * factor * factor + b * factor).max(0.0),
    })
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    Brace,
    Yield,
    Roll,
    Slide,
    Rest,
}
#[derive(Clone, Copy, Debug)]
pub struct Recovery {
    pub phase: Phase,
    pub age: f32,
    pub attempts: u8,
    pub direction: f32,
    pub work: f64,
    elapsed: f32,
    best: f32,
    cooldown: f32,
    progress: f32,
    quiet: f32,
}
impl Default for Recovery {
    fn default() -> Self {
        Self {
            phase: Phase::Brace,
            age: 0.0,
            attempts: 0,
            direction: 1.0,
            work: 0.0,
            elapsed: 0.0,
            best: -1.0,
            cooldown: 0.0,
            progress: 0.0,
            quiet: 0.0,
        }
    }
}
#[derive(Clone, Copy)]
pub struct Sense {
    pub supported: bool,
    pub speed: f32,
    pub spin: f32,
    pub limb_spin: f32,
    pub supine: f32,
    pub roll_side: f32,
}
#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    pub brace_seconds: f32,
    pub roll_max_speed: f32,
    pub roll_max_spin: f32,
    pub attempt_seconds: f32,
    pub retry_seconds: f32,
    pub max_attempts: u8,
    pub max_work_j: f64,
    pub power_w: f64,
}
impl Recovery {
    pub fn roll_strength(&self) -> f32 {
        if self.phase == Phase::Roll {
            ((self.elapsed - 0.15) / 0.35).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
    pub fn obstructed(&mut self, retry_seconds: f32) {
        self.phase = Phase::Yield;
        self.elapsed = 0.0;
        self.cooldown = retry_seconds;
        self.direction = -self.direction;
    }
    pub fn advance(&mut self, dt: f32, s: Sense, p: &Policy) -> Phase {
        let dt = dt.clamp(0.0, 0.05);
        self.age += dt;
        self.cooldown = (self.cooldown - dt).max(0.0);
        let manageable = s.supported && s.speed <= p.roll_max_speed && s.spin <= p.roll_max_spin;
        self.quiet = if s.supported && s.speed < 0.35 && s.spin < 0.8 && s.limb_spin < 1.2 {
            self.quiet + dt
        } else {
            0.0
        };
        if self.quiet > 0.8
            && (s.supine > 0.65 || self.attempts >= p.max_attempts || self.age > 3.5)
        {
            self.phase = Phase::Rest;
            return self.phase;
        }
        let brace_seconds = p.brace_seconds * (1.0 - (s.speed / 60.0).clamp(0.0, 0.8) * 0.65);
        if self.age < brace_seconds {
            self.phase = Phase::Brace;
            return self.phase;
        }
        if !manageable {
            if self.phase == Phase::Roll {
                self.cooldown = p.retry_seconds;
                self.direction = -self.direction;
            }
            self.phase = Phase::Yield;
            self.elapsed = 0.0;
            return self.phase;
        }
        // Even inexpensive damping/slide corrections have a finite lifecycle;
        // otherwise they can repeatedly wake a supported limb forever.
        let active_lifetime = p.brace_seconds
            + f32::from(p.max_attempts) * (p.attempt_seconds + p.retry_seconds)
            + 1.0;
        if self.age >= active_lifetime || self.work >= p.max_work_j - 1e-8 {
            self.phase = Phase::Yield;
            return self.phase;
        }
        if s.supine > 0.7 {
            self.phase = Phase::Slide;
            self.elapsed = 0.0;
            return self.phase;
        }
        if self.phase == Phase::Roll {
            self.elapsed += dt;
            if s.supine > self.best + 0.06 {
                self.best = s.supine;
                self.progress = self.elapsed;
            }
            if self.elapsed >= p.attempt_seconds || self.elapsed - self.progress >= 0.85 {
                self.phase = Phase::Yield;
                self.cooldown = p.retry_seconds;
                self.elapsed = 0.0;
                self.direction = -self.direction;
            }
        } else if self.cooldown <= 0.0 && self.attempts < p.max_attempts {
            self.phase = Phase::Roll;
            self.attempts += 1;
            self.elapsed = 0.0;
            self.best = s.supine;
            self.progress = 0.0;
            if self.attempts == 1 {
                self.direction = if s.roll_side < 0.0 { -1.0 } else { 1.0 };
            }
        } else {
            self.phase = Phase::Yield;
        }
        self.phase
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const ID: Tensor = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    #[test]
    fn tensor_impulse_preserves_pair_momentum_and_caps_actual_work() {
        let ca = [[0.5, 0.1, 0.0], [0.1, 1.0, 0.0], [0.0, 0.0, 2.0]];
        let pa = [[0.2, 0.0, 0.03], [0.0, 0.4, 0.0], [0.03, 0.0, 0.8]];
        for budget in [0.0, 0.01, 0.3, 2.0] {
            let wa = [1.0, -2.0, 0.5];
            let wb = [-0.4, 1.0, 0.3];
            let r = drive(wa, wb, ca, pa, [4.0, 1.0, -3.0], 0.7, budget).unwrap();
            let angular = std::array::from_fn(|i| {
                mul(inverse(ca).unwrap(), r.child_delta)[i]
                    + mul(inverse(pa).unwrap(), r.parent_delta)[i]
            });
            assert!(dot(angular, angular) < 1e-20);
            let exact = dot(r.impulse, std::array::from_fn(|i| wa[i] - wb[i]))
                + 0.5
                    * dot(
                        r.impulse,
                        std::array::from_fn(|i| r.child_delta[i] - r.parent_delta[i]),
                    );
            assert!(exact <= budget + 1e-10 && r.positive_work <= budget + 1e-10);
            assert!(dot(r.impulse, r.impulse).sqrt() <= 0.700001);
        }
    }
    #[test]
    fn damping_needs_no_positive_work_and_invalid_inputs_are_rejected() {
        let r = drive([2.0, 0.0, 0.0], [0.0; 3], ID, ID, [0.0; 3], 1.0, 0.0).unwrap();
        assert_eq!(r.positive_work, 0.0);
        assert!(r.child_delta[0] < 0.0);
        assert!(drive([f64::NAN; 3], [0.0; 3], ID, ID, [0.0; 3], 1.0, 1.0).is_none());
        assert!(
            drive(
                [0.0; 3],
                [0.0; 3],
                [[0.0; 3]; 3],
                [[0.0; 3]; 3],
                [0.0; 3],
                1.0,
                1.0
            )
            .is_none()
        );
    }
    #[test]
    fn fixed_support_guide_caps_torque_and_exact_work() {
        for budget in [0.0, 0.01, 0.5, 2.0] {
            let w = [0.4, 0.2, -0.3];
            let r = support_drive(w, ID, [0.4, 0.2, 3.0], 0.8, budget).unwrap();
            let work = dot(r.impulse, w) + 0.5 * dot(r.impulse, r.child_delta);
            assert!(work <= budget + 1e-12 && r.positive_work <= budget + 1e-12);
            assert!(dot(r.impulse, r.impulse).sqrt() <= 0.800001);
            assert_eq!(r.parent_delta, [0.0; 3]);
        }
    }
    #[test]
    fn recovery_yields_to_airborne_speed_and_spin_and_has_finite_retries() {
        let p = Policy {
            brace_seconds: 0.3,
            roll_max_speed: 22.0,
            roll_max_spin: 7.0,
            attempt_seconds: 0.8,
            retry_seconds: 0.4,
            max_attempts: 2,
            max_work_j: 35.0,
            power_w: 25.0,
        };
        let mut r = Recovery::default();
        let mut s = Sense {
            supported: false,
            speed: 8.0,
            spin: 2.0,
            limb_spin: 2.0,
            supine: -1.0,
            roll_side: -1.0,
        };
        for _ in 0..60 {
            r.advance(1.0 / 60.0, s, &p);
        }
        assert_eq!(r.phase, Phase::Yield);
        assert_eq!(r.attempts, 0);
        s.supported = true;
        s.speed = 50.0;
        r.advance(0.02, s, &p);
        assert_eq!(r.phase, Phase::Yield);
        s.speed = 8.0;
        s.spin = 20.0;
        r.advance(0.02, s, &p);
        assert_eq!(r.phase, Phase::Yield);
        s.spin = 2.0;
        for _ in 0..240 {
            r.advance(1.0 / 60.0, s, &p);
        }
        assert_eq!(r.attempts, 2);
        assert_eq!(r.phase, Phase::Yield);
        s.supine = 0.9;
        r.advance(0.02, s, &p);
        assert_eq!(r.phase, Phase::Yield, "even cheap slide drive must expire");
        r = Recovery {
            age: 0.4,
            ..Recovery::default()
        };
        r.advance(0.02, s, &p);
        assert_eq!(r.phase, Phase::Slide);
        s.speed = 0.0;
        s.spin = 0.0;
        s.limb_spin = 0.0;
        for _ in 0..60 {
            r.advance(1.0 / 60.0, s, &p);
        }
        assert_eq!(r.phase, Phase::Rest);
    }
}
