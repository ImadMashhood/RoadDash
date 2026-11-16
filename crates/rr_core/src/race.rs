//! Playable race bounds in absolute physical-route chainage.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RaceInterval {
    pub start: f32,
    pub finish: f32,
}
impl RaceInterval {
    pub const METROPOLIS: Self = Self {
        start: 500.,
        finish: 10500.,
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
        assert_eq!(r.length(), 10000.);
        assert_eq!(r.fraction(212.), 0.);
        assert_eq!(r.fraction(500.), 0.);
        assert_eq!(r.fraction(5500.), 0.5);
        assert_eq!(r.fraction(12000.), 1.);
        assert_eq!(r.grid_station(-288.), 212.);
        assert_eq!(r.grid_station(-8.) - r.grid_station(-288.), 280.);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WasteCause {
    Bike,
    Rider,
    River,
}

#[derive(Clone, Copy, Debug)]
pub struct RaceHealth {
    pub bike: f32,
    pub rider: f32,
}

impl Default for RaceHealth {
    fn default() -> Self {
        Self {
            bike: 100.,
            rider: 100.,
        }
    }
}

impl RaceHealth {
    pub fn damage(&mut self, bike: f32, rider: f32) {
        self.bike = (self.bike - bike).max(0.);
        self.rider = (self.rider - rider).max(0.);
    }

    pub fn struck(&mut self, hit_bike: bool, damage: crate::weapons::ConditionDamage) {
        if hit_bike {
            self.damage(damage.bike, 0.);
        } else {
            self.damage(0., damage.rider);
        }
    }

    pub fn crash(&mut self, speed_mps: f32) {
        let severity = (speed_mps / 45.).clamp(0., 1.);
        self.damage(9. + 23. * severity, 4. + 12. * severity);
    }

    pub fn cause(self) -> Option<WasteCause> {
        if self.bike <= 0. {
            Some(WasteCause::Bike)
        } else if self.rider <= 0. {
            Some(WasteCause::Rider)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy)]
pub struct RaceRank {
    pub grid_id: usize,
    pub laps: u32,
    pub progress: f32,
    pub finish: Option<f32>,
    pub wasted: bool,
}

impl RaceRank {
    pub fn standings_cmp(self, other: Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::{Greater, Less};
        self.wasted
            .cmp(&other.wasted)
            .then_with(|| match (self.finish, other.finish) {
                (Some(a), Some(b)) => a.total_cmp(&b),
                (Some(_), None) => Less,
                (None, Some(_)) => Greater,
                (None, None) => other
                    .laps
                    .cmp(&self.laps)
                    .then_with(|| other.progress.total_cmp(&self.progress)),
            })
            .then_with(|| self.grid_id.cmp(&other.grid_id))
    }
}
