//! Persistent prop durability. Never applies velocity or impulses to racers.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Damage {
    pub spent_j: f32,
    pub revision: u32,
    pub broken: bool,
}
impl Damage {
    pub fn hit(&mut self, energy_j: f32, capacity_j: f32) -> bool {
        if self.broken
            || !energy_j.is_finite()
            || energy_j <= 0.
            || !capacity_j.is_finite()
            || capacity_j <= 0.
        {
            return false;
        }
        self.spent_j = (self.spent_j + energy_j).min(capacity_j);
        self.revision = self.revision.wrapping_add(1);
        self.broken = self.spent_j >= capacity_j;
        self.broken
    }
    pub fn reset(&mut self) {
        let revision = self.revision.wrapping_add(1);
        *self = Self {
            revision,
            ..Self::default()
        };
    }
}
pub fn impact_energy(mass_kg: f32, closing_mps: f32, min_speed: f32) -> f32 {
    if !mass_kg.is_finite() || !closing_mps.is_finite() || mass_kg <= 0. || closing_mps < min_speed
    {
        return 0.;
    }
    0.5 * mass_kg.min(2000.) * closing_mps.min(100.).powi(2)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn damage_accumulates_without_healing_or_duplicate_break() {
        let mut d = Damage::default();
        assert!(!d.hit(30., 100.));
        assert!(!d.hit(40., 100.));
        assert_eq!(d.spent_j, 70.);
        assert!(d.hit(40., 100.));
        let revision = d.revision;
        assert!(!d.hit(900., 100.));
        assert_eq!(d.revision, revision);
        d.reset();
        assert!(!d.broken);
        assert_eq!(d.spent_j, 0.);
    }
    #[test]
    fn energy_ignores_slow_nudges_and_invalid_inputs() {
        assert_eq!(impact_energy(180., 0.28, 4.), 0.);
        assert_eq!(impact_energy(180., 10., 4.), 9000.);
        assert_eq!(impact_energy(f32::NAN, 10., 4.), 0.);
        assert_eq!(impact_energy(10., f32::INFINITY, 4.), 0.);
    }
}
