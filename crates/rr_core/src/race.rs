//! Playable race bounds in absolute physical-route chainage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaceInterval {
    pub start: f32,
    pub finish: f32,
}
impl RaceInterval {
    pub const METROPOLIS: Self = Self {
        start: 500.,
        finish: 11500.,
    };
    pub fn length(self) -> f32 {
        self.finish - self.start
    }
    pub fn fraction(self, progress: f32) -> f32 {
        ((progress - self.start) / self.length()).clamp(0., 1.)
    }
    pub fn grid_station(self, offset: f32) -> f32 {
        self.start + offset
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn race_distance_excludes_both_physical_tails_and_preserves_grid_spacing() {
        let r = RaceInterval::METROPOLIS;
        assert_eq!(r.length(), 11000.);
        assert_eq!(r.fraction(212.), 0.);
        assert_eq!(r.fraction(500.), 0.);
        assert_eq!(r.fraction(6000.), 0.5);
        assert_eq!(r.fraction(12000.), 1.);
        assert_eq!(r.grid_station(-288.), 212.);
        assert_eq!(r.grid_station(-8.) - r.grid_station(-288.), 280.);
    }
}
