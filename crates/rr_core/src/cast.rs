//! Original gameplay interpretation of the approved portrait dossiers.
//! Occupation, riding and fighting descriptions drive traits, never ethnicity or gender.
use crate::weapons::Weapon;
#[derive(Clone, Copy, Debug)]
pub struct Character {
    pub portrait: usize,
    pub weapon: Option<Weapon>,
    pub aggressive: bool,
    pub pace: f32,
    pub patience: f32,
    pub kick: bool,
}
const fn c(
    portrait: usize,
    weapon: Option<Weapon>,
    aggressive: bool,
    pace: f32,
    patience: f32,
    kick: bool,
) -> Character {
    Character {
        portrait,
        weapon,
        aggressive,
        pace,
        patience,
        kick,
    }
}
// Fourteen grid slots preserve the race contract. George and Elif remain bar cast.
pub const CAST: [Character; 14] = [
    c(0, None, false, 0.96, 1.5, false),
    c(1, None, true, 0.99, 0.9, true),
    c(2, Some(Weapon::Club), false, 0.95, 1.2, false),
    c(3, Some(Weapon::Whip), true, 1.01, 0.8, false),
    c(4, None, false, 1.04, 1.6, false),
    c(5, None, true, 1.02, 0.9, true),
    c(6, Some(Weapon::RollingPin), false, 0.98, 1.1, false),
    c(7, None, true, 1.03, 0.7, false),
    c(8, Some(Weapon::Chain), false, 0.97, 1.3, false),
    c(9, Some(Weapon::Club), true, 1.0, 0.8, false),
    c(10, Some(Weapon::BaseballBat), false, 1.05, 1.5, false),
    c(11, Some(Weapon::Chain), true, 1.02, 1.0, false),
    c(12, None, false, 1.06, 1.7, false),
    c(15, Some(Weapon::Club), true, 1.0, 0.75, false),
];
pub fn character(id: usize) -> Option<Character> {
    id.checked_sub(1).and_then(|i| CAST.get(i)).copied()
}
pub const NAMES: [&str; 15] = [
    "YOU",
    "Yamakha",
    "Tyrone White",
    "Esteban Gonzalez",
    "Osama bin Countryistani",
    "Naveen Gowda Nambiar",
    "Ellen Windyke",
    "Luigi’s Pizzeria",
    "Pee Pi Long",
    "Hog",
    "Cleetus Mayballs",
    "Abeyomi",
    "Sha’queesha",
    "Anastasia Jakmiova",
    "Dolph Hiller",
];

/// Fixed-tick obstruction recovery; never teleports or reverses into an occupied gap.
#[derive(Clone, Debug, Default)]
pub struct DriveMemory {
    stalled: f32,
    backing: f32,
    queue_reposition: bool,
    cooldown: f32,
    pub lane: Option<f32>,
}
impl DriveMemory {
    pub fn cancel_reverse(&mut self) {
        self.backing = 0.;
        self.queue_reposition = false;
    }
    pub fn reversing(&self) -> bool {
        self.backing > 0.
    }
    pub fn step(&mut self, dt: f32, speed: f32, wants_progress: bool, rear_clear: bool) -> bool {
        self.step_with_queue(dt, speed, wants_progress, rear_clear, false, false)
    }
    /// Preserve the ordinary short unsticking pulse. A compressed queue instead
    /// keeps a bounded launch-space goal, terminating as soon as it is satisfied.
    pub fn step_with_queue(
        &mut self,
        dt: f32,
        speed: f32,
        wants_progress: bool,
        rear_clear: bool,
        request_queue: bool,
        needs_space: bool,
    ) -> bool {
        self.cooldown = (self.cooldown - dt).max(0.);
        if self.backing > 0. {
            self.backing = if rear_clear && (!self.queue_reposition || needs_space) {
                (self.backing - dt).max(0.)
            } else {
                0.
            };
            self.stalled = 0.;
            return self.reversing();
        }
        self.stalled = if speed.abs() < 0.65 && (wants_progress || request_queue) {
            self.stalled + dt
        } else {
            0.
        };
        if self.stalled > 2.5 && rear_clear && self.cooldown == 0. {
            self.queue_reposition = request_queue;
            self.backing = if request_queue { 4.5 } else { 1.25 };
            self.cooldown = 6.;
            self.stalled = 0.;
        }
        self.reversing()
    }
    pub fn stable_lane(&mut self, desired: f32, dt: f32, urgency: bool) -> f32 {
        let lane = self.lane.get_or_insert(desired);
        *lane += (desired - *lane).clamp(
            -dt * if urgency { 4.0 } else { 1.2 },
            dt * if urgency { 4.0 } else { 1.2 },
        );
        *lane
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn portrait_names_and_loadouts_match_dossiers() {
        for (i, c) in CAST.iter().enumerate() {
            assert_eq!(NAMES[i + 1], crate::frontend::ROSTER[c.portrait].name);
        }
        assert_eq!(CAST.iter().filter(|c| c.aggressive).count(), 7);
        assert_eq!(CAST[3].weapon, Some(Weapon::Whip));
        assert_eq!(CAST[10].weapon, Some(Weapon::BaseballBat));
    }
    #[test]
    fn queue_backoff_holds_goal_but_stops_for_space_rear_obstruction_and_time_bound() {
        for stop_for_gap in [false, true] {
            let mut m = DriveMemory::default();
            for _ in 0..151 {
                m.step_with_queue(1. / 60., 0., false, true, true, true);
            }
            assert!(m.reversing());
            for _ in 0..120 {
                assert!(m.step_with_queue(1. / 60., -2., false, true, false, true));
            }
            assert!(!m.step_with_queue(1. / 60., -2., false, stop_for_gap, false, !stop_for_gap));
            for _ in 0..120 {
                assert!(!m.step_with_queue(1. / 60., 0., false, true, true, true));
            }
        }
        let mut m = DriveMemory::default();
        for _ in 0..151 {
            m.step_with_queue(1. / 60., 0., false, true, true, true);
        }
        let mut ticks = 0;
        while m.step_with_queue(1. / 60., -2., false, true, false, true) {
            ticks += 1;
            assert!(ticks <= 270);
        }
        assert!(ticks >= 268);
    }
    #[test]
    fn blocked_rider_backs_out_but_not_at_a_queue_or_into_a_car() {
        let mut m = DriveMemory::default();
        for _ in 0..240 {
            assert!(!m.step(1. / 60., 0., false, true));
        }
        for _ in 0..240 {
            assert!(!m.step(1. / 60., 0., true, false));
        }
        assert!(m.step(1. / 60., 0., true, true));
        assert!(!m.step(1. / 60., -1., true, false));
        for _ in 0..120 {
            assert!(!m.step(1. / 60., 0., true, true));
        }
    }
}
