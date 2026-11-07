//! Posture-aware native-motor intent. Physics contacts and native joints retain authority.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Tumble,
    Protect,
    Roll,
    Slide,
    Rest,
}
#[derive(Clone, Copy, Debug)]
pub struct Input {
    pub supported: bool,
    pub near_ground: bool,
    pub speed: f32,
    pub spin: f32,
    pub facing_up: f32,
    pub side_up: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct Reaction {
    pub mode: Mode,
    pub age: f32,
    pub attempt_age: f32,
    pub attempts: u8,
    pub direction: f32,
    pub work_left: f32,
    cooldown: f32,
    best_facing: f32,
    stuck: f32,
}
impl Reaction {
    pub fn new(work: f32) -> Self {
        Self {
            mode: Mode::Tumble,
            age: 0.0,
            attempt_age: 0.0,
            attempts: 0,
            direction: 1.0,
            work_left: work,
            cooldown: 0.0,
            best_facing: -1.0,
            stuck: 0.0,
        }
    }
    pub fn step(
        &mut self,
        dt: f32,
        i: Input,
        max_speed: f32,
        max_spin: f32,
        attempt_seconds: f32,
        max_attempts: u8,
    ) -> Mode {
        let dt = dt.clamp(0.0, 0.05);
        self.age += dt;
        self.cooldown = (self.cooldown - dt).max(0.0);
        if !i.speed.is_finite() || !i.spin.is_finite() || self.work_left <= 0.0 {
            if self.mode == Mode::Roll {
                self.cooldown = 0.7;
            }
            self.mode = Mode::Tumble;
            return self.mode;
        }
        if i.speed > max_speed || i.spin > max_spin {
            if self.mode == Mode::Roll {
                self.cooldown = 0.7;
            }
            self.mode = if i.near_ground && self.age < 0.45 && i.spin < max_spin * 1.5 {
                Mode::Protect
            } else {
                Mode::Tumble
            };
            return self.mode;
        }
        if i.supported && i.speed < 0.35 && i.spin < 0.6 && i.facing_up > 0.65 {
            self.mode = Mode::Rest;
            return self.mode;
        }
        if i.supported && i.facing_up > 0.65 {
            self.mode = Mode::Slide;
            self.attempt_age = 0.0;
            return self.mode;
        }
        if self.mode == Mode::Roll {
            self.attempt_age += dt;
            if i.facing_up > self.best_facing + 0.02 {
                self.best_facing = i.facing_up;
                self.stuck = 0.0;
            } else {
                self.stuck += dt;
            }
            if !i.supported || self.attempt_age >= attempt_seconds || self.stuck > 0.7 {
                self.mode = Mode::Tumble;
                self.cooldown = 0.7;
            }
        } else if i.supported
            && self.age > 0.3
            && self.cooldown == 0.0
            && self.attempts < max_attempts
        {
            self.direction = if i.side_up.abs() > 0.15 {
                i.side_up.signum()
            } else if self.attempts == 0 {
                1.0
            } else {
                -1.0
            };
            self.attempts += 1;
            self.attempt_age = 0.0;
            self.best_facing = i.facing_up;
            self.stuck = 0.0;
            self.mode = Mode::Roll;
        } else if i.near_ground && self.age < 1.1 {
            self.mode = Mode::Protect;
        } else {
            self.mode = Mode::Tumble;
        }
        self.mode
    }
    /// Bound motor effort using maximum active torque times measured relative speed.
    /// Existing physics energy envelope independently rejects unbudgeted kinetic gains.
    pub fn activation(&mut self, requested: f32, rate: f32, dt: f32) -> f32 {
        let cost = (rate.max(0.0) * requested * dt).max(0.0);
        let scale = if cost > 0.0 {
            (self.work_left / cost).min(1.0)
        } else {
            1.0
        };
        self.work_left = (self.work_left - cost * scale).max(0.0);
        requested * scale
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Input {
        Input {
            supported: true,
            near_ground: true,
            speed: 4.0,
            spin: 1.0,
            facing_up: -1.0,
            side_up: 0.8,
        }
    }
    #[test]
    fn yields_airborne_fast_and_wall_contacts_and_never_forces_success() {
        let mut r = Reaction::new(100.0);
        for _ in 0..30 {
            r.step(1.0 / 60.0, input(), 15.0, 6.0, 1.2, 2);
        }
        assert_eq!(r.mode, Mode::Roll);
        let mut i = input();
        i.supported = false;
        i.near_ground = false;
        assert_eq!(r.step(0.02, i, 15.0, 6.0, 1.2, 2), Mode::Tumble);
        i.supported = true;
        i.speed = 60.0;
        assert_eq!(r.step(0.02, i, 15.0, 6.0, 1.2, 2), Mode::Tumble);
    }
    #[test]
    fn finite_attempts_and_supine_slide_do_not_require_a_scripted_rotation() {
        let mut r = Reaction::new(100.0);
        for _ in 0..600 {
            r.step(1.0 / 60.0, input(), 15.0, 6.0, 0.5, 2);
        }
        assert_eq!(r.attempts, 2);
        assert_eq!(r.mode, Mode::Tumble);
        let mut i = input();
        i.facing_up = 0.9;
        assert_eq!(r.step(0.02, i, 15.0, 6.0, 0.5, 2), Mode::Slide);
        let a = r.activation(0.5, 100.0, 1.0);
        assert!(a <= 0.5);
        r.activation(0.5, 1000.0, 1.0);
        assert_eq!(r.work_left, 0.0);
        assert_eq!(r.activation(0.5, 1000.0, 1.0), 0.0);
    }
}
