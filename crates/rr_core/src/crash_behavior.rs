//! Fixed-tick loss-of-balance intent; no transform or velocity ownership.
#[derive(Clone, Copy, Debug)]
pub struct Stagger {
    pub age: f32,
    pub away: f32,
    pub origin: Option<[f32; 3]>,
    pub right: [f32; 3],
}
impl Stagger {
    pub fn new(away: f32) -> Self {
        Self {
            age: 0.0,
            origin: None,
            right: [0.0; 3],
            away: if away < 0.0 { -1.0 } else { 1.0 },
        }
    }
    pub fn advance(&mut self, dt: f32, error: f32, lateral_speed: f32) -> (f32, bool) {
        self.age += dt.clamp(0.0, 0.05);
        let steer = if self.age < 0.28 {
            self.away * (self.age / 0.12).min(1.0) * 0.72
        } else {
            (-self.away * 0.9 - error * 0.9 - lateral_speed * 0.3).clamp(-1.0, 1.0)
        };
        (
            steer,
            (self.age >= 0.58 && lateral_speed * self.away < -0.2)
                || (self.age >= 0.78 && error.abs() < 0.08)
                || self.age >= 1.2,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stagger_countersteers_then_releases_equally_at_fixed_rates() {
        for side in [-1.0, 1.0] {
            let mut s = Stagger::new(side);
            let mut released = false;
            for tick in 0..60 {
                let (steer, release) = s.advance(1.0 / 60.0, 0.0, 0.0);
                if tick == 10 {
                    assert!(steer * side > 0.0);
                }
                if tick == 24 {
                    assert!(steer * side < 0.0);
                }
                if release {
                    assert!((0.78..0.82).contains(&s.age));
                    released = true;
                    break;
                }
            }
            assert!(released);
        }
    }
}
